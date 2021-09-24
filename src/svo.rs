pub struct Svo {
    children: Vec<Svo>,
    size: i32,
    id: i32,
}

impl Svo {
    pub fn new(array: &[u8], width: i32, height: i32) -> Svo {

        //-y -x -z to -y -x z to -y x -z to -y x z
        //y -x -z to y -x z to y x -z to y x z
        let mut bottom_back_left = Vec::new();
        let mut bottom_back_right = Vec::new();
        let mut bottom_front_left = Vec::new();
        let mut bottom_front_right = Vec::new();
        let mut top_back_left = Vec::new();
        let mut top_back_right = Vec::new();
        let mut top_front_left = Vec::new();
        let mut top_front_right = Vec::new();

        //Catogrize data
        let mut children: Vec<Svo> = Vec::new();
        for y in 0..height {
            for z in 0..width {
                for x in 0..width {
                    if array[((x + width * (z + width * y)) * 4) as usize + 3] > 0 {
                        if x < width / 2 && y < width / 2 && z < width / 2 {
                            bottom_back_left.push(cgmath::Vector3::new(x, y, z));
                        }
                        else if x >= width / 2 && y < width / 2 && z < width / 2 {
                            bottom_back_right.push(cgmath::Vector3::new(x, y, z));
                        }
                        else if x < width / 2 && y < width / 2 && z >= width / 2 {
                            bottom_front_left.push(cgmath::Vector3::new(x, y, z));
                        }
                        else if x >= width / 2 && y < width / 2 && z >= width / 2 {
                            bottom_front_right.push(cgmath::Vector3::new(x, y, z));
                        }
                        else if x < width / 2 && y >= width / 2 && z < width / 2 {
                            top_back_left.push(cgmath::Vector3::new(x, y, z));
                        }
                        else if x >= width / 2 && y >= width / 2 && z < width / 2 {
                            top_back_right.push(cgmath::Vector3::new(x, y, z));
                        }
                        else if x < width / 2 && y >= width / 2 && z >= width / 2 {
                            top_front_left.push(cgmath::Vector3::new(x, y, z));
                        }
                        else if x >= width / 2 && y >= width / 2 && z >= width / 2 {
                            top_front_right.push(cgmath::Vector3::new(x, y, z));
                        }
                    }
                }
            }
        }

        //println!("{} {} {} {}", bottom_back_left.len(), bottom_back_right.len(), bottom_front_left.len(), bottom_front_right.len());

        let size = width;
        let mut id = 1;
        if bottom_back_left.len() == 0 && bottom_back_right.len() == 0 && bottom_front_left.len() == 0 && bottom_front_right.len() == 0 && 
            top_back_left.len() == 0 && top_back_right.len() == 0 && top_front_left.len() == 0 && top_front_right.len() == 0 {
            id = 0;
        }else {

            let (svo0, id0) = Svo::new_node(&bottom_back_left, width / 2, 0, 0, 0);
            let (svo1, id1) = Svo::new_node(&bottom_back_right, width / 2, size / 2, 0, 0);
            let (svo2, id2) = Svo::new_node(&bottom_front_left, width / 2, 0, 0, size / 2);
            let (svo3, id3) = Svo::new_node(&bottom_front_right, width / 2, size / 2, 0, size / 2);
            let (svo4, id4) = Svo::new_node(&top_back_left, width / 2, 0, size / 2, 0);
            let (svo5, id5) = Svo::new_node(&top_back_right, width / 2, size / 2, size / 2, 0);
            let (svo6, id6) = Svo::new_node(&top_front_left, width / 2, 0, size / 2, size / 2);
            let (svo7, id7) = Svo::new_node(&top_front_right, width / 2, size / 2, size / 2, size / 2);

            children.push(svo0);
            children.push(svo1);
            children.push(svo2);
            children.push(svo3);
            children.push(svo4);
            children.push(svo5);
            children.push(svo6);
            children.push(svo7);

            if id0 == 2 && id1 == 2 && id2 == 2 && id3 == 2 && id4 == 2 && id5 == 2 && id6 == 2 && id7 == 2 {
                id = 2;
            }
        }

        Svo { children, size, id }
    }

    pub fn new_node(array: &Vec::<cgmath::Vector3<i32>>, width: i32, mut offset_x: i32, mut offset_y: i32, mut offset_z: i32) -> (Svo, i32) {

        if width == 1 && array.len() != 0 {
            //println!("{} {} {}", offset_x, offset_y, offset_z);
            return (Svo { children: Vec::new(), size: 1, id: 2 }, 2);
        }

        if array.len() == 0 {
            return (Svo { children: Vec::new(), size: width, id: 0 }, 0);
        }

        //-y -x -z to -y -x z to -y x -z to -y x z
        //y -x -z to y -x z to y x -z to y x z
        let mut bottom_back_left = Vec::new();
        let mut bottom_back_right = Vec::new();
        let mut bottom_front_left = Vec::new();
        let mut bottom_front_right = Vec::new();
        let mut top_back_left = Vec::new();
        let mut top_back_right = Vec::new();
        let mut top_front_left = Vec::new();
        let mut top_front_right = Vec::new();

        //Catogrize data
        let mut children: Vec<Svo> = Vec::new();
        for i in 0..array.len() {

            if array[i].x - offset_x < width / 2 && array[i].y - offset_y < width / 2 && array[i].z - offset_z < width / 2 {
                bottom_back_left.push(array[i]);
            }
            else if array[i].x - offset_x >= width / 2 && array[i].y - offset_y < width / 2 && array[i].z - offset_z < width / 2 {
                bottom_back_right.push(array[i]);
            }
            else if array[i].x - offset_x < width / 2 && array[i].y - offset_y < width / 2 && array[i].z - offset_z >= width / 2 {
                bottom_front_left.push(array[i]);
            }
            else if array[i].x - offset_x >= width / 2 && array[i].y - offset_y < width / 2 && array[i].z - offset_z >= width / 2 {
                bottom_front_right.push(array[i]);
            }
            else if array[i].x - offset_x < width / 2 && array[i].y - offset_y >= width / 2 && array[i].z - offset_z < width / 2 {
                top_back_left.push(array[i]);
            }
            else if array[i].x - offset_x >= width / 2 && array[i].y - offset_y >= width / 2 && array[i].z - offset_z < width / 2 {
                top_back_right.push(array[i]);
            }
            else if array[i].x - offset_x < width / 2 && array[i].y - offset_y >= width / 2 && array[i].z - offset_z >= width / 2 {
                top_front_left.push(array[i]);
            }
            else if array[i].x - offset_x >= width / 2 && array[i].y - offset_y >= width / 2 && array[i].z - offset_z >= width / 2 {
                top_front_right.push(array[i]);
            }        
        }


        let size = width;
        let mut id = 1;
        if bottom_back_left.len() == 0 && bottom_back_right.len() == 0 && bottom_front_left.len() == 0 && bottom_front_right.len() == 0 && 
            top_back_left.len() == 0 && top_back_right.len() == 0 && top_front_left.len() == 0 && top_front_right.len() == 0 {
            id = 0;
        }
        else {
            
            let (svo0, id0) = Svo::new_node(&bottom_back_left, width / 2, offset_x, offset_y, offset_z);
            let (svo1, id1) = Svo::new_node(&bottom_back_right, width / 2, offset_x + width / 2, offset_y, offset_z);
            let (svo2, id2) = Svo::new_node(&bottom_front_left, width / 2, offset_x, offset_y, offset_z + width / 2);
            let (svo3, id3) = Svo::new_node(&bottom_front_right, width / 2, offset_x + width / 2, offset_y, offset_z + width / 2);
            let (svo4, id4) = Svo::new_node(&top_back_left, width / 2, offset_x, offset_y + size / 2, offset_z);
            let (svo5, id5) = Svo::new_node(&top_back_right, width / 2, offset_x + size / 2, offset_y + size / 2, offset_z);
            let (svo6, id6) = Svo::new_node(&top_front_left, width / 2, offset_x, offset_y + size / 2, offset_z + size / 2);
            let (svo7, id7) = Svo::new_node(&top_front_right, width / 2, offset_x + size / 2, offset_y + size / 2, offset_z + size / 2);

            if id0 == 2 && id1 == 2 && id2 == 2 && id3 == 2 && id4 == 2 && id5 == 2 && id6 == 2 && id7 == 2 {
                id = 2;
                children.push(Svo { children: Vec::new(), size: width / 2, id: 2 });
                children.push(Svo { children: Vec::new(), size: width / 2, id: 2 });
                children.push(Svo { children: Vec::new(), size: width / 2, id: 2 });
                children.push(Svo { children: Vec::new(), size: width / 2, id: 2 });
                children.push(Svo { children: Vec::new(), size: width / 2, id: 2 });
                children.push(Svo { children: Vec::new(), size: width / 2, id: 2 });
                children.push(Svo { children: Vec::new(), size: width / 2, id: 2 });
                children.push(Svo { children: Vec::new(), size: width / 2, id: 2 });
            }
            else {

                children.push(svo0);
                children.push(svo1);
                children.push(svo2);
                children.push(svo3);
                children.push(svo4);
                children.push(svo5);
                children.push(svo6);
                children.push(svo7);
            }
        }

        (Svo { children, size, id }, id)
    }

    pub fn check_solid(&self, pos: &cgmath::Vector3<f32>, mut offset_x: f32, mut offset_y: f32, mut offset_z: f32, info: &mut i32) {

        if self.id == 0 {
            //println!("In empty");
            return;
        }

        if self.id == 2 {
            //println!("In solid");
            *info = 1;
            return;
        }

        if self.id == 1 && self.size == 1 {
            //println!("shouldnt make it here");
            return;
        }

        //println!("min {} {} {}", offset_x, offset_y, offset_z);
        //println!("max {} {} {}", offset_x + self.size as f32, offset_y + self.size as f32, offset_z + self.size as f32);

        if pos.x - offset_x < (self.size / 2) as f32 && pos.y - offset_y < (self.size / 2) as f32 && pos.z - offset_z < (self.size / 2) as f32 {
            self.children[0].check_solid(pos, offset_x, offset_y, offset_z, info);
            return;
        }
        else if pos.x - offset_x >= (self.size / 2) as f32 && pos.y - offset_y < (self.size / 2) as f32 && pos.z - offset_z < (self.size / 2) as f32 {
            offset_x += (self.size / 2) as f32;
            self.children[1].check_solid(pos, offset_x, offset_y, offset_z, info);
            return;
        }
        else if pos.x - offset_x < (self.size / 2) as f32 && pos.y - offset_y < (self.size / 2) as f32 && pos.z - offset_z >= (self.size / 2) as f32 {
            offset_z += (self.size / 2) as f32;
            self.children[2].check_solid(pos, offset_x, offset_y, offset_z, info);
            return;
        }
        else if pos.x - offset_x >= (self.size / 2) as f32 && pos.y - offset_y < (self.size / 2) as f32 && pos.z - offset_z >= (self.size / 2) as f32 {
            offset_x += (self.size / 2) as f32;
            offset_z += (self.size / 2) as f32;
            self.children[3].check_solid(pos, offset_x, offset_y, offset_z, info);
            return;
        }
        else if pos.x - offset_x < (self.size / 2) as f32 && pos.y - offset_y >= (self.size / 2) as f32 && pos.z - offset_z < (self.size / 2) as f32 {
            offset_y += (self.size / 2) as f32;
            self.children[4].check_solid(pos, offset_x, offset_y, offset_z, info);
            return;
        }
        else if pos.x - offset_x >= (self.size / 2) as f32 && pos.y - offset_y >= (self.size / 2) as f32 && pos.z - offset_z < (self.size / 2) as f32 {
            offset_y += (self.size / 2) as f32;
            offset_x += (self.size / 2) as f32;
            self.children[5].check_solid(pos, offset_x, offset_y, offset_z, info);
            return;
        }
        else if pos.x - offset_x < (self.size / 2) as f32 && pos.y - offset_y >= (self.size / 2) as f32 && pos.z - offset_z >= (self.size / 2) as f32 {
            offset_y += (self.size / 2) as f32;
            offset_z += (self.size / 2) as f32;
            self.children[6].check_solid(pos, offset_x, offset_y, offset_z, info);
            return;
        }
        else if pos.x - offset_x >= (self.size / 2) as f32 && pos.y - offset_y >= (self.size / 2) as f32 && pos.z - offset_z >= (self.size / 2) as f32 {
            offset_z += (self.size / 2) as f32;
            offset_x += (self.size / 2) as f32;
            offset_y += (self.size / 2) as f32;

            self.children[7].check_solid(pos, offset_x, offset_y, offset_z, info);
            return;
        }

        //println!("Outside bounds");
        return;
    }

    pub fn num_nodes(&self, count: &mut i32) {

        *count += 1;

        for i in 0..self.children.len() {
            self.children[i].num_nodes(count);
        }
    }

    pub fn compile_svo_data(&self, data: &mut Vec<i32>) {

        if self.id == 2 || self.id == 0 {
            data.push(self.id);
            data.push(self.size);
        }
        else if self.id == 1 {
            data.push(self.id);
            data.push(self.size);
            let insert_point = data.len();
            data.push(0);
            data.push(0);
            data.push(0);
            data.push(0);
            data.push(0);
            data.push(0);
            data.push(0);
            for i in 0..self.children.len() {
                let offset_val = data.len() as i32;
                self.children[i].compile_svo_data(data);
                if i > 0 {
                    data[insert_point + (i - 1)] = offset_val;
                }
            }
        }
    }
}