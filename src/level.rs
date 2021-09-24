pub struct Level {
    val: i32,
}

//https://github.com/yvt/openspades/blob/f141208322953a4473eb42019c578e87a8a36640/Sources/Client/GameMap.cpp

impl Level {
    pub fn new(map_name: &str) {

        let dir = "./res/".to_string() + map_name + ".vxl";

        let mut bytes = std::fs::read(dir).unwrap();

        let mut pos = 0;

        let mut map: Vec<u8> = Vec::with_capacity(512 * 512 * 64 * 4);
        map.resize(512 * 512 * 64 * 4, 0);

        for z in 0..512 {
            for x in 0..512 {

                let mut y = 0;

                loop {

                    let number_4byte = bytes[pos];
                    let top_colour_start = bytes[pos + 1] as u32;
                    let top_colour_end = bytes[pos + 2] as u32;

                    for i in y..top_colour_start {
                        map[x + 512 * (z + 512 * i) as usize * 4] = 0;
                        map[x + 512 * (z + 512 * i) as usize * 4 + 1] = 0;
                        map[x + 512 * (z + 512 * i) as usize * 4 + 2] = 0;
                        map[x + 512 * (z + 512 * i) as usize * 4 + 3] = 0;
                    }

                    for y in top_colour_start..(top_colour_end + 1) {
                        let col = *bytemuck::from_bytes::<u32>(&bytes[(pos + 4)..(pos + 8)]);
                        //println!("{} {} {} {}", bytes[pos + 4], bytes[pos + 5], bytes[pos + 6], bytes[pos + 7]);
                        map[x + 512 * (z + 512 * y) as usize * 4] = col as u8;
                        map[x + 512 * (z + 512 * y) as usize * 4 + 1] = 0;
                        map[x + 512 * (z + 512 * y) as usize * 4 + 2] = 0;
                        map[x + 512 * (z + 512 * y) as usize * 4 + 3] = 0;
                    }

                    let len_bottom = top_colour_end - top_colour_start + 1;

                    if number_4byte == 0 {
                        pos += 4 * (len_bottom + 1) as usize;
                        break;
                    }

                    pos += bytes[pos] as usize * 4;
                }
            }
        }
    }
}