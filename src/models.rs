// Basic Stellar model
pub struct Star {
    pub mass: f32,         //·mass·of·star·in·M_sol¬
    pub radius: f32,       //·radius·of·star·in·R_sol¬
    pub luminosity: f32,   //·luminosity·of·star·in·L_sol¬
    pub temp: u32,         //·temperature·of·star·in·K¬
    pub specclass: String, //·spectral·class·of·star¬
    pub lumclass: String,  //·luminosity·class·of·star¬
    pub coords: Coords,    //·coordinates·of·star·wrt·center¬
    pub age: f32,          //·age·of·the·star·in·Gyr¬
    pub lifespan: f32,     //·calculated·main·sequence·lifespan·of·the·star·in·Gyr¬
    // pub flags,          //·flags·added·during·aging·pass¬
}

// Cartesian Coords
pub struct Coords {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

// Spherical Coords
pub struct SphereCoords {
    pub r: f64,
    pub i: f64,
    pub a: f64,
}
