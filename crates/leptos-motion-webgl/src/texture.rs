//! Texture system for WebGL rendering

use crate::error::{Result, WebGLError};
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::{HtmlImageElement, ImageData, WebGl2RenderingContext, WebGlTexture};

/// Texture format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextureFormat {
    RGB,
    RGBA,
    Luminance,
    LuminanceAlpha,
    Alpha,
    Depth,
    DepthStencil,
}

impl TextureFormat {
    /// Get the WebGL internal format
    pub fn to_webgl_internal_format(&self) -> u32 {
        match self {
            TextureFormat::RGB => WebGl2RenderingContext::RGB,
            TextureFormat::RGBA => WebGl2RenderingContext::RGBA,
            TextureFormat::Luminance => WebGl2RenderingContext::LUMINANCE,
            TextureFormat::LuminanceAlpha => WebGl2RenderingContext::LUMINANCE_ALPHA,
            TextureFormat::Alpha => WebGl2RenderingContext::ALPHA,
            TextureFormat::Depth => WebGl2RenderingContext::DEPTH_COMPONENT,
            TextureFormat::DepthStencil => WebGl2RenderingContext::DEPTH_STENCIL,
        }
    }

    /// Get the WebGL format
    pub fn to_webgl_format(&self) -> u32 {
        match self {
            TextureFormat::RGB => WebGl2RenderingContext::RGB,
            TextureFormat::RGBA => WebGl2RenderingContext::RGBA,
            TextureFormat::Luminance => WebGl2RenderingContext::LUMINANCE,
            TextureFormat::LuminanceAlpha => WebGl2RenderingContext::LUMINANCE_ALPHA,
            TextureFormat::Alpha => WebGl2RenderingContext::ALPHA,
            TextureFormat::Depth => WebGl2RenderingContext::DEPTH_COMPONENT,
            TextureFormat::DepthStencil => WebGl2RenderingContext::DEPTH_STENCIL,
        }
    }
}

/// Texture type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextureType {
    UnsignedByte,
    UnsignedShort565,
    UnsignedShort4444,
    UnsignedShort5551,
    UnsignedInt,
    Float,
    HalfFloat,
}

impl TextureType {
    /// Get the WebGL type
    pub fn to_webgl_type(&self) -> u32 {
        match self {
            TextureType::UnsignedByte => WebGl2RenderingContext::UNSIGNED_BYTE,
            TextureType::UnsignedShort565 => WebGl2RenderingContext::UNSIGNED_SHORT_5_6_5,
            TextureType::UnsignedShort4444 => WebGl2RenderingContext::UNSIGNED_SHORT_4_4_4_4,
            TextureType::UnsignedShort5551 => WebGl2RenderingContext::UNSIGNED_SHORT_5_5_5_1,
            TextureType::UnsignedInt => WebGl2RenderingContext::UNSIGNED_INT,
            TextureType::Float => WebGl2RenderingContext::FLOAT,
            TextureType::HalfFloat => WebGl2RenderingContext::HALF_FLOAT,
        }
    }
}

/// Texture wrapping mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextureWrapping {
    Repeat,
    ClampToEdge,
    MirroredRepeat,
}

impl TextureWrapping {
    /// Get the WebGL wrapping mode
    pub fn to_webgl_wrapping(&self) -> u32 {
        match self {
            TextureWrapping::Repeat => WebGl2RenderingContext::REPEAT,
            TextureWrapping::ClampToEdge => WebGl2RenderingContext::CLAMP_TO_EDGE,
            TextureWrapping::MirroredRepeat => WebGl2RenderingContext::MIRRORED_REPEAT,
        }
    }
}

/// Texture filtering mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextureFiltering {
    Nearest,
    Linear,
    NearestMipmapNearest,
    LinearMipmapNearest,
    NearestMipmapLinear,
    LinearMipmapLinear,
}

impl TextureFiltering {
    /// Get the WebGL filtering mode
    pub fn to_webgl_filtering(&self) -> u32 {
        match self {
            TextureFiltering::Nearest => WebGl2RenderingContext::NEAREST,
            TextureFiltering::Linear => WebGl2RenderingContext::LINEAR,
            TextureFiltering::NearestMipmapNearest => {
                WebGl2RenderingContext::NEAREST_MIPMAP_NEAREST
            }
            TextureFiltering::LinearMipmapNearest => WebGl2RenderingContext::LINEAR_MIPMAP_NEAREST,
            TextureFiltering::NearestMipmapLinear => WebGl2RenderingContext::NEAREST_MIPMAP_LINEAR,
            TextureFiltering::LinearMipmapLinear => WebGl2RenderingContext::LINEAR_MIPMAP_LINEAR,
        }
    }
}

/// Texture configuration
#[derive(Debug, Clone)]
pub struct TextureConfig {
    /// Texture format
    pub format: TextureFormat,
    /// Texture type
    pub texture_type: TextureType,
    /// Wrap mode for S coordinate
    pub wrap_s: TextureWrapping,
    /// Wrap mode for T coordinate
    pub wrap_t: TextureWrapping,
    /// Minification filter
    pub min_filter: TextureFiltering,
    /// Magnification filter
    pub mag_filter: TextureFiltering,
    /// Generate mipmaps
    pub generate_mipmaps: bool,
    /// Flip Y coordinate
    pub flip_y: bool,
    /// Premultiply alpha
    pub premultiply_alpha: bool,
}

impl Default for TextureConfig {
    fn default() -> Self {
        Self {
            format: TextureFormat::RGBA,
            texture_type: TextureType::UnsignedByte,
            wrap_s: TextureWrapping::Repeat,
            wrap_t: TextureWrapping::Repeat,
            min_filter: TextureFiltering::LinearMipmapLinear,
            mag_filter: TextureFiltering::Linear,
            generate_mipmaps: true,
            flip_y: true,
            premultiply_alpha: false,
        }
    }
}

/// Texture data source
#[derive(Debug, Clone)]
pub enum TextureSource {
    /// Image element
    Image(HtmlImageElement),
    /// Image data
    ImageData(ImageData),
    /// Raw pixel data
    Pixels(Vec<u8>),
    /// Canvas element
    Canvas(web_sys::HtmlCanvasElement),
    /// Video element
    Video(web_sys::HtmlVideoElement),
}

/// Texture information
#[derive(Debug, Clone)]
pub struct TextureInfo {
    /// Texture width
    pub width: u32,
    /// Texture height
    pub height: u32,
    /// Texture format
    pub format: TextureFormat,
    /// Texture type
    pub texture_type: TextureType,
    /// Number of channels
    pub channels: u32,
    /// Size in bytes
    pub size_bytes: usize,
}

/// WebGL texture wrapper
#[derive(Debug)]
pub struct Texture {
    /// Texture ID
    pub id: String,
    /// Texture name
    pub name: String,
    /// WebGL texture object
    pub texture: WebGlTexture,
    /// Texture configuration
    pub config: TextureConfig,
    /// Texture information
    pub info: TextureInfo,
    /// Loaded flag
    pub loaded: bool,
    /// Error message if loading failed
    pub error: Option<String>,
}

impl Texture {
    /// Create a new texture
    pub fn new(
        id: String,
        name: String,
        texture: WebGlTexture,
        config: TextureConfig,
        info: TextureInfo,
    ) -> Self {
        Self {
            id,
            name,
            texture,
            config,
            info,
            loaded: true,
            error: None,
        }
    }

    /// Create a texture from image element
    pub fn from_image(
        context: &WebGl2RenderingContext,
        name: &str,
        image: HtmlImageElement,
        config: Option<TextureConfig>,
    ) -> Result<Self> {
        let config = config.unwrap_or_default();
        let width = image.width() as u32;
        let height = image.height() as u32;

        let info = TextureInfo {
            width,
            height,
            format: config.format,
            texture_type: config.texture_type,
            channels: match config.format {
                TextureFormat::RGB => 3,
                TextureFormat::RGBA => 4,
                TextureFormat::Luminance => 1,
                TextureFormat::LuminanceAlpha => 2,
                TextureFormat::Alpha => 1,
                _ => 4,
            },
            size_bytes: (width
                * height
                * match config.format {
                    TextureFormat::RGB => 3,
                    TextureFormat::RGBA => 4,
                    TextureFormat::Luminance => 1,
                    TextureFormat::LuminanceAlpha => 2,
                    TextureFormat::Alpha => 1,
                    _ => 4,
                }) as usize,
        };

        let texture = context
            .create_texture()
            .ok_or_else(|| WebGLError::texture_error("Failed to create texture"))?;

        context.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&texture));

        // Set texture parameters
        context.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_WRAP_S,
            config.wrap_s.to_webgl_wrapping() as i32,
        );
        context.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_WRAP_T,
            config.wrap_t.to_webgl_wrapping() as i32,
        );
        context.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_MIN_FILTER,
            config.min_filter.to_webgl_filtering() as i32,
        );
        context.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_MAG_FILTER,
            config.mag_filter.to_webgl_filtering() as i32,
        );

        // Upload texture data
        context.tex_image_2d_with_u32_and_u32_and_html_image_element(
            WebGl2RenderingContext::TEXTURE_2D,
            0,
            config.format.to_webgl_internal_format() as i32,
            config.format.to_webgl_format(),
            config.texture_type.to_webgl_type(),
            &image,
        )?;

        // Generate mipmaps if requested
        if config.generate_mipmaps {
            context.generate_mipmap(WebGl2RenderingContext::TEXTURE_2D);
        }

        context.bind_texture(WebGl2RenderingContext::TEXTURE_2D, None);

        Ok(Self::new(
            uuid::Uuid::new_v4().to_string(),
            name.to_string(),
            texture,
            config,
            info,
        ))
    }

    /// Create a texture from image data
    pub fn from_image_data(
        context: &WebGl2RenderingContext,
        name: &str,
        image_data: ImageData,
        config: Option<TextureConfig>,
    ) -> Result<Self> {
        let config = config.unwrap_or_default();
        let width = image_data.width() as u32;
        let height = image_data.height() as u32;

        let info = TextureInfo {
            width,
            height,
            format: config.format,
            texture_type: config.texture_type,
            channels: 4, // ImageData is always RGBA
            size_bytes: (width * height * 4) as usize,
        };

        let texture = context
            .create_texture()
            .ok_or_else(|| WebGLError::texture_error("Failed to create texture"))?;

        context.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&texture));

        // Set texture parameters
        context.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_WRAP_S,
            config.wrap_s.to_webgl_wrapping() as i32,
        );
        context.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_WRAP_T,
            config.wrap_t.to_webgl_wrapping() as i32,
        );
        context.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_MIN_FILTER,
            config.min_filter.to_webgl_filtering() as i32,
        );
        context.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_MAG_FILTER,
            config.mag_filter.to_webgl_filtering() as i32,
        );

        // Upload texture data
        context.tex_image_2d_with_u32_and_u32_and_image_data(
            WebGl2RenderingContext::TEXTURE_2D,
            0,
            config.format.to_webgl_internal_format() as i32,
            config.format.to_webgl_format(),
            config.texture_type.to_webgl_type(),
            &image_data,
        )?;

        // Generate mipmaps if requested
        if config.generate_mipmaps {
            context.generate_mipmap(WebGl2RenderingContext::TEXTURE_2D);
        }

        context.bind_texture(WebGl2RenderingContext::TEXTURE_2D, None);

        Ok(Self::new(
            uuid::Uuid::new_v4().to_string(),
            name.to_string(),
            texture,
            config,
            info,
        ))
    }

    /// Create a texture from raw pixel data
    pub fn from_pixels(
        context: &WebGl2RenderingContext,
        name: &str,
        pixels: Vec<u8>,
        width: u32,
        height: u32,
        config: Option<TextureConfig>,
    ) -> Result<Self> {
        let config = config.unwrap_or_default();

        let info = TextureInfo {
            width,
            height,
            format: config.format,
            texture_type: config.texture_type,
            channels: match config.format {
                TextureFormat::RGB => 3,
                TextureFormat::RGBA => 4,
                TextureFormat::Luminance => 1,
                TextureFormat::LuminanceAlpha => 2,
                TextureFormat::Alpha => 1,
                _ => 4,
            },
            size_bytes: pixels.len(),
        };

        let texture = context
            .create_texture()
            .ok_or_else(|| WebGLError::texture_error("Failed to create texture"))?;

        context.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&texture));

        // Set texture parameters
        context.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_WRAP_S,
            config.wrap_s.to_webgl_wrapping() as i32,
        );
        context.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_WRAP_T,
            config.wrap_t.to_webgl_wrapping() as i32,
        );
        context.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_MIN_FILTER,
            config.min_filter.to_webgl_filtering() as i32,
        );
        context.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_MAG_FILTER,
            config.mag_filter.to_webgl_filtering() as i32,
        );

        // Upload texture data
        context.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
            WebGl2RenderingContext::TEXTURE_2D,
            0,
            config.format.to_webgl_internal_format() as i32,
            width as i32,
            height as i32,
            0,
            config.format.to_webgl_format(),
            config.texture_type.to_webgl_type(),
            Some(&pixels),
        )?;

        // Generate mipmaps if requested
        if config.generate_mipmaps {
            context.generate_mipmap(WebGl2RenderingContext::TEXTURE_2D);
        }

        context.bind_texture(WebGl2RenderingContext::TEXTURE_2D, None);

        Ok(Self::new(
            uuid::Uuid::new_v4().to_string(),
            name.to_string(),
            texture,
            config,
            info,
        ))
    }

    /// Bind the texture to a texture unit
    pub fn bind(&self, context: &WebGl2RenderingContext, texture_unit: u32) -> Result<()> {
        context.active_texture(WebGl2RenderingContext::TEXTURE0 + texture_unit);
        context.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&self.texture));
        Ok(())
    }

    /// Unbind the texture
    pub fn unbind(&self, context: &WebGl2RenderingContext) {
        context.bind_texture(WebGl2RenderingContext::TEXTURE_2D, None);
    }

    /// Get texture size
    pub fn get_size(&self) -> (u32, u32) {
        (self.info.width, self.info.height)
    }

    /// Check if texture is loaded
    pub fn is_loaded(&self) -> bool {
        self.loaded
    }

    /// Get error message if any
    pub fn get_error(&self) -> Option<&String> {
        self.error.as_ref()
    }
}

/// Texture manager for loading and caching textures
pub struct TextureManager {
    /// WebGL context
    context: Rc<WebGl2RenderingContext>,
    /// Loaded textures
    textures: HashMap<String, Texture>,
    /// Loading queue
    loading_queue: Vec<String>,
    /// Maximum texture size
    max_texture_size: u32,
    /// Maximum texture units
    max_texture_units: u32,
}

impl TextureManager {
    /// Create a new texture manager
    pub fn new(context: &WebGl2RenderingContext) -> Result<Self> {
        let max_texture_size = context
            .get_parameter(WebGl2RenderingContext::MAX_TEXTURE_SIZE)
            .unwrap_or_else(|_| 0.into())
            .as_f64()
            .unwrap_or(0.0) as u32;

        let max_texture_units = context
            .get_parameter(WebGl2RenderingContext::MAX_TEXTURE_IMAGE_UNITS)
            .unwrap_or_else(|_| 0.into())
            .as_f64()
            .unwrap_or(0.0) as u32;

        Ok(Self {
            context: Rc::new(context.clone()),
            textures: HashMap::new(),
            loading_queue: Vec::new(),
            max_texture_size,
            max_texture_units,
        })
    }

    /// Load a texture from URL
    pub fn load_from_url(
        &mut self,
        name: &str,
        url: &str,
        config: Option<TextureConfig>,
    ) -> Result<()> {
        // TODO: Implement async texture loading from URL
        // This would involve creating an Image element and loading the URL
        Err(WebGLError::texture_error(
            "Async texture loading not yet implemented",
        ))
    }

    /// Create a texture from image element
    pub fn create_from_image(
        &mut self,
        name: &str,
        image: HtmlImageElement,
        config: Option<TextureConfig>,
    ) -> Result<()> {
        let texture = Texture::from_image(&self.context, name, image, config)?;
        self.textures.insert(name.to_string(), texture);
        Ok(())
    }

    /// Create a texture from image data
    pub fn create_from_image_data(
        &mut self,
        name: &str,
        image_data: ImageData,
        config: Option<TextureConfig>,
    ) -> Result<()> {
        let texture = Texture::from_image_data(&self.context, name, image_data, config)?;
        self.textures.insert(name.to_string(), texture);
        Ok(())
    }

    /// Create a texture from raw pixels
    pub fn create_from_pixels(
        &mut self,
        name: &str,
        pixels: Vec<u8>,
        width: u32,
        height: u32,
        config: Option<TextureConfig>,
    ) -> Result<()> {
        let texture = Texture::from_pixels(&self.context, name, pixels, width, height, config)?;
        self.textures.insert(name.to_string(), texture);
        Ok(())
    }

    /// Get a texture by name
    pub fn get_texture(&self, name: &str) -> Option<&Texture> {
        self.textures.get(name)
    }

    /// Get a mutable texture by name
    pub fn get_texture_mut(&mut self, name: &str) -> Option<&mut Texture> {
        self.textures.get_mut(name)
    }

    /// Remove a texture
    pub fn remove_texture(&mut self, name: &str) -> Option<Texture> {
        if let Some(texture) = self.textures.remove(name) {
            self.context.delete_texture(Some(&texture.texture));
            Some(texture)
        } else {
            None
        }
    }

    /// Get all texture names
    pub fn get_texture_names(&self) -> Vec<String> {
        self.textures.keys().cloned().collect()
    }

    /// Get texture count
    pub fn get_texture_count(&self) -> usize {
        self.textures.len()
    }

    /// Get maximum texture size
    pub fn get_max_texture_size(&self) -> u32 {
        self.max_texture_size
    }

    /// Get maximum texture units
    pub fn get_max_texture_units(&self) -> u32 {
        self.max_texture_units
    }

    /// Clear all textures
    pub fn clear(&mut self) {
        for (_, texture) in self.textures.drain() {
            self.context.delete_texture(Some(&texture.texture));
        }
    }
}

impl Drop for TextureManager {
    fn drop(&mut self) {
        self.clear();
    }
}
