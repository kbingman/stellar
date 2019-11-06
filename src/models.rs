// Basic Stellar model
pub struct Star {
    pub mass: f64,            //·mass·of·star·in·M_sol¬
    pub radius: f64,          //·radius·of·star·in·R_sol¬
    pub luminosity: f64,      //·luminosity·of·star·in·L_sol¬
    pub temp: u32,            //·temperature·of·star·in·K¬
    pub spectral_class: String,  //·spectral·class·of·star¬
    // pub lumclass: String,  //·luminosity·class·of·star¬
    // pub coords: Coords,    //·coordinates·of·star·wrt·center¬
    // pub age: f64,          //·age·of·the·star·in·Gyr¬
    // pub lifespan: f64,     //·calculated·main·sequence·lifespan·of·the·star·in·Gyr¬
    // pub flags,             //·flags·added·during·aging·pass¬
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
