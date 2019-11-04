// Basic Stellar model
pub struct Star {
    pub mass: u32,          //·mass·of·star·in·M_sol¬
    pub radius: u32,        //·radius·of·star·in·R_sol¬
    pub luminosity: u32,    //·luminosity·of·star·in·L_sol¬
    pub temp: u32,          //·temperature·of·star·in·K¬
    pub specclass: String, //·spectral·class·of·star¬
    pub lumclass: u32,      //·luminosity·class·of·star¬
    // pub coords[3];       //·coordinates·of·star·wrt·center¬
    pub age: u32,           //·age·of·the·star·in·Gyr¬
    pub lifespan: u32,      //·calculated·main·sequence·lifespan·of·the·star·in·Gyr¬
    // pub flags;            //·flags·added·during·aging·pass¬
}

