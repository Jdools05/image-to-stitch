


pub struct ColorThread {
    floss: String,
    name: String,
    hex: String,
}

impl ColorThread {
    pub fn new(floss: String, name: String, hex: String) -> ColorThread {
        ColorThread {
            floss,
            name,
            hex,
        }
    }

    pub fn get_floss(&self) -> &String {
        &self.floss
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_hex(&self) -> &String {
        &self.hex
    }

    pub fn load_from_file(file_path: &str) -> Vec<ColorThread> {
        // want to load from a csv
        // format: Floss,DMC Name,R,G,B,Hex
        // example: 1,White,255,255,255,#FFFFFF
        let mut color_threads: Vec<ColorThread> = Vec::new();

        let mut rdr = csv::Reader::from_path(file_path).unwrap();
        for result in rdr.records() {
            let record = result.unwrap();
            // ignore the first record
            let floss: String = record[0].parse::<String>().unwrap();
            let name: String = record[1].parse::<String>().unwrap();
            let hex: String = record[5].parse::<String>().unwrap();
            color_threads.push(ColorThread::new(floss, name, hex));
        }

        color_threads
    }
}

