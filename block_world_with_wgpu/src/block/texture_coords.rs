use rust_game_library_wgpu::objects::tex_coord::TexCoord;

pub struct TextureCoords {
    pub up: TexCoord,
    pub down: TexCoord,
    pub north: TexCoord,
    pub east: TexCoord,
    pub south: TexCoord,
    pub west: TexCoord
}

impl TextureCoords {
    pub fn new_default() -> TextureCoords{
        let default = TexCoord::default();

        return TextureCoords {
            up: default.clone(),
            down: default.clone(),
            north: default.clone(),
            east: default.clone(),
            south: default.clone(),
            west: default.clone()
        }
    }

    pub fn new_from_one(tex_coord: TexCoord) -> TextureCoords{
        let default = tex_coord.clone();

        return TextureCoords {
            up: default.clone(),
            down: default.clone(),
            north: default.clone(),
            east: default.clone(),
            south: default.clone(),
            west: default.clone()
        }
    }

    pub fn new_sides_top(sides: TexCoord,top: TexCoord) -> TextureCoords{
        let default = sides.clone();

        return TextureCoords {
            up: top,
            down: default.clone(),
            north: default.clone(),
            east: default.clone(),
            south: default.clone(),
            west: default.clone()
        }
    }

    pub fn new_sides_top_bottom(sides: TexCoord,top: TexCoord,down: TexCoord) -> TextureCoords{
        let default = sides.clone();

        return TextureCoords {
            up: top,
            down,
            north: default.clone(),
            east: default.clone(),
            south: default.clone(),
            west: default.clone()
        }
    }
}