use crate::io::assets_reader::schemas::models::{
    Display, Element, GuiLightMode, Model, Texture, Textures,
};

/// Methods for resolving the properties of a [`Model`] with respect to its
/// parents.
pub struct ModelResolver;

impl ModelResolver {
    /// Iterates through a [`Model`] and all of its parents to resolve all of
    /// the model's properties in a way that reflects the intended inheritance
    /// and/or override behavior of the Minecraft model format.
    ///
    /// The method takes in an iterator of [`Model`]s where the first element is
    /// the model being resolved, and the subsequent elements (if any) are the
    /// chain of parents of that model.
    ///
    /// # Example
    ///
    /// ```
    /// # use minecraft_assets::api::{ModelResolver};
    /// use maplit::hashmap;
    ///
    /// use minecraft_assets::schemas::models::*;
    ///
    /// let parent = Model {
    ///     textures: Some(Textures::from(hashmap! {
    ///         "up" => "#side",
    ///         "down" => "#side"
    ///     })),
    ///     elements: Some(vec![
    ///         Element {
    ///             faces: hashmap! {
    ///                 BlockFace::Up => ElementFace {
    ///                     texture: Texture::from("#up"),
    ///                     ..Default::default()
    ///                 },
    ///                 BlockFace::Down => ElementFace {
    ///                     texture: Texture::from("#down"),
    ///                     ..Default::default()
    ///                 },
    ///                 BlockFace::East => ElementFace {
    ///                     texture: Texture::from("#side"),
    ///                     ..Default::default()
    ///                 },
    ///                 BlockFace::West => ElementFace {
    ///                     texture: Texture::from("#side"),
    ///                     ..Default::default()
    ///                 }
    ///             },
    ///             ..Default::default()
    ///         }
    ///     ]),
    ///     ..Default::default()
    /// };
    ///
    /// let child = Model {
    ///     textures: Some(Textures::from(hashmap! {
    ///         "up" => "textures/up",
    ///         "side" => "textures/side"
    ///     })),
    ///     ..Default::default()
    /// };
    ///
    /// let expected = Model {
    ///     textures: Some(Textures::from(hashmap! {
    ///         "up" => "textures/up",
    ///         "down" => "textures/side",
    ///         "side" => "textures/side"
    ///     })),
    ///     elements: Some(vec![
    ///         Element {
    ///             faces: hashmap! {
    ///                 BlockFace::Up => ElementFace {
    ///                     texture: Texture::from("textures/up"),
    ///                     ..Default::default()
    ///                 },
    ///                 BlockFace::Down => ElementFace {
    ///                     texture: Texture::from("textures/side"),
    ///                     ..Default::default()
    ///                 },
    ///                 BlockFace::East => ElementFace {
    ///                     texture: Texture::from("textures/side"),
    ///                     ..Default::default()
    ///                 },
    ///                 BlockFace::West => ElementFace {
    ///                     texture: Texture::from("textures/side"),
    ///                     ..Default::default()
    ///                 }
    ///             },
    ///             ..Default::default()
    ///         }
    ///     ]),
    ///     ..Default::default()
    /// };
    ///
    /// let resolved = ModelResolver::resolve_model([&child, &parent].into_iter());
    ///
    /// assert_eq!(resolved, expected);
    /// ```
    pub fn resolve_model<'a>(models: impl IntoIterator<Item = &'a Model> + Clone) -> Model {
        let textures = Self::resolve_textures(models.clone());
        let mut elements = Self::resolve_elements(models.clone());

        if let Some(ref mut elements) = elements {
            Self::resolve_element_textures(elements, &textures);
        }

        let display = Self::resolve_display(models.clone());
        let ambient_occlusion = Self::resolve_ambient_occlusion(models.clone());
        let gui_light_mode = Self::resolve_gui_light_mode(models.clone());
        let overrides = models.into_iter().next().unwrap().overrides.clone();

        Model {
            parent: None,
            display,
            textures: Some(textures),
            elements,
            ambient_occlusion,
            gui_light_mode,
            overrides,
        }
    }

    /// Iterates through a [`Model`] and all of its parents to resolve all of
    /// the model's [texture variables].
    ///
    /// This works by merging together the [`Textures`] maps from all models in
    /// the parent-child chain, and then substituting texture variables with
    /// concrete values where possible.
    ///
    /// [texture variables]: Textures#texture-variables
    ///
    /// # Example
    ///
    /// ```
    /// # use minecraft_assets::api::{ModelResolver};
    /// use maplit::hashmap;
    ///
    /// use minecraft_assets::schemas::models::{Model, Textures};
    ///
    /// let child = Model {
    ///     textures: Some(Textures::from(hashmap! {
    ///         "child_texture" => "textures/child",
    ///         "bar" => "#parent_texture"
    ///     })),
    ///     ..Default::default()
    /// };
    ///
    /// let parent = Model {
    ///     textures: Some(Textures::from(hashmap! {
    ///         "parent_texture" => "textures/parent",
    ///         "foo" => "#child_texture"
    ///     })),
    ///     ..Default::default()
    /// };
    ///
    /// // Provide models in increasing level of parenthood.
    /// let models = [child, parent];
    /// let resolved = ModelResolver::resolve_textures(models.iter());
    ///
    /// let expected = Textures::from(hashmap! {
    ///     "parent_texture" => "textures/parent",
    ///     "foo" => "textures/child",              // <------- resolved
    ///     "child_texture" => "textures/child",
    ///     "bar" => "textures/parent"              // <------- resolved    
    /// });
    ///
    /// assert_eq!(resolved, expected);
    /// ```
    pub fn resolve_textures<'a>(models: impl IntoIterator<Item = &'a Model>) -> Textures {
        let mut textures = Textures::default();

        for model in models.into_iter() {
            if let Some(mut parent_textures) = model.textures.clone() {
                // Resolve variables in the parent using the child textures first.
                parent_textures.resolve(&textures);

                // Then resolve variables in the child using the parent textures.
                textures.resolve(&parent_textures);

                // Merge the **child** into the parent.
                std::mem::swap(&mut textures, &mut parent_textures);
                textures.merge(parent_textures.clone());
            }
        }

        textures
    }

    /// Iterates through a [`Model`] and all of its parents to resolve the
    /// model's cuboid [`Element`]s.
    ///
    /// This works by taking the first set of elements present in the chain of
    /// parents. Unlike textures, child definitions for model elements
    /// completely override elements from the parent(s).
    ///
    /// # Example
    ///
    /// ```
    /// # use minecraft_assets::api::{ModelResolver};
    /// use minecraft_assets::schemas::models::{Model, Element};
    ///
    /// let element1 = Element {
    ///     from: [0.0, 0.0, 0.0],
    ///     to: [1.0, 1.0, 1.0],
    ///     ..Default::default()
    /// };
    ///
    /// let element2 = Element {
    ///     from: [5.0, 6.0, 7.0],
    ///     to: [4.0, 3.0, 2.0],
    ///     ..Default::default()
    /// };
    ///
    /// let model1 = Model {
    ///     elements: Some(vec![element1.clone()]),
    ///     ..Default::default()
    /// };
    ///
    /// let model2 = Model {
    ///     elements: Some(vec![element2.clone()]),
    ///     ..Default::default()
    /// };
    ///
    /// let empty = Model::default();
    ///
    /// let resolved = ModelResolver::resolve_elements([&empty, &model1].into_iter());
    /// assert_eq!(resolved, Some(vec![element1.clone()]));
    ///
    /// let resolved = ModelResolver::resolve_elements([&empty, &model2].into_iter());
    /// assert_eq!(resolved, Some(vec![element2.clone()]));
    ///
    /// let resolved = ModelResolver::resolve_elements([&model1, &model2].into_iter());
    /// assert_eq!(resolved, Some(vec![element1.clone()]));
    ///
    /// let resolved = ModelResolver::resolve_elements([&model2, &model1].into_iter());
    /// assert_eq!(resolved, Some(vec![element2.clone()]));
    ///
    /// let resolved = ModelResolver::resolve_elements([&empty, &empty].into_iter());
    /// assert_eq!(resolved, None);
    /// ```
    pub fn resolve_elements<'a>(
        models: impl IntoIterator<Item = &'a Model>,
    ) -> Option<Vec<Element>> {
        Self::first_model_where_some(models, |model| model.elements.as_ref()).cloned()
    }

    /// Iterates through each [`ElementFace`] in each [`Element`] and resolves
    /// any texture variables using the provided map.
    ///
    /// [`ElementFace`]: crate::schemas::models::ElementFace
    pub fn resolve_element_textures<'a>(
        elements: impl IntoIterator<Item = &'a mut Element>,
        textures: &Textures,
    ) {
        for element in elements.into_iter() {
            for face in element.faces.values_mut() {
                if let Some(substitution) = face.texture.resolve(textures) {
                    face.texture = Texture::from(substitution);
                }
            }
        }
    }

    /// Iterates through a [`Model`] and all of its parents to resolve the
    /// model's [`Display`] properties.
    ///
    /// Similar to [`elements`] works by taking the first set of properties
    /// present in the chain of parents.
    ///
    /// [`elements`]: Self::resolve_elements
    pub fn resolve_display<'a>(models: impl IntoIterator<Item = &'a Model>) -> Option<Display> {
        Self::first_model_where_some(models, |model| model.display.as_ref()).cloned()
    }

    /// Iterates through a [`Model`] and all of its parents to resolve the
    /// model's ambient occlusion setting.
    ///
    /// Similar to [`elements`] works by taking the first property value present
    /// in the chain of parents.
    ///
    /// [`elements`]: Self::resolve_elements
    pub fn resolve_ambient_occlusion<'a>(
        models: impl IntoIterator<Item = &'a Model>,
    ) -> Option<bool> {
        Self::first_model_where_some(models, |model| model.ambient_occlusion.as_ref()).copied()
    }

    /// Iterates through a [`Model`] and all of its parents to resolve the
    /// model's GUI light mode setting.
    ///
    /// Similar to [`elements`] works by taking the first property value present
    /// in the chain of parents.
    ///
    /// [`elements`]: Self::resolve_elements
    pub fn resolve_gui_light_mode<'a>(
        models: impl IntoIterator<Item = &'a Model>,
    ) -> Option<GuiLightMode> {
        Self::first_model_where_some(models, |model| model.gui_light_mode.as_ref()).copied()
    }

    fn first_model_where_some<'a, F, T>(
        models: impl IntoIterator<Item = &'a Model>,
        mut op: F,
    ) -> Option<&'a T>
    where
        F: FnMut(&'a Model) -> Option<&'a T>,
    {
        for model in models.into_iter() {
            if let Some(item) = op(model) {
                return Some(item);
            }
        }

        None
    }
}
