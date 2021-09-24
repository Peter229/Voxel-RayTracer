#version 450

layout(location = 0) out vec4 f_color;

layout(set = 1, binding = 0) uniform texture3D world_tex;
layout(set = 1, binding = 1) uniform sampler world_sam;

layout(set = 2, binding = 0) buffer svo {
    int svo_data[]; 
};


layout(set = 0, binding = 0)
uniform Uniforms {
    ivec2 screen_di;
    vec3 cam_pos;
    vec3 fwd;
    vec3 rgt;
    vec3 up;
};

struct Sphere {
    vec3 center;
    float radius;
    vec3 col;
};

struct Cube {
    vec3 mi;
    vec3 ma;
};

struct HitInfo {
    float tmin;
    float tmax;
};

int check_svo_v_pos(vec3 pos);
Cube box_from_svo(vec3 pos);
HitInfo ray_box(vec3 pos, vec3 n_inv, Cube c);

void main() {

    const int width = 256;
    const int height = 256;

    float fov = 1.57;

    vec2 u = (gl_FragCoord.xy - vec2(1280.0, 720.0) * 0.5) / 720.0;

    vec3 dir = normalize(fwd + fov * (u.x * rgt + u.y * up));

    f_color = vec4(0.1, 0.1, 0.5, 1.0);

    //DDA 3D
    vec3 ray_unit_step_size = vec3(abs(1.0 / dir.x), abs(1.0 / dir.y), abs(1.0 / dir.z));
    if (dir.x == 0.0) {
        ray_unit_step_size.x = 0.0;
    }
    if (dir.y == 0.0) {
        ray_unit_step_size.x = 0.0;
    }
    if (dir.z == 0.0) {
        ray_unit_step_size.z = 0.0;
    }
    ivec3 map_check = ivec3(int(cam_pos.x), int(cam_pos.y), int(cam_pos.z));
    vec3 ray_length = vec3(0.0, 0.0, 0.0);
    ivec3 step_i = ivec3(0, 0, 0);

    if (dir.x < 0.0) {
        step_i.x = -1;
        ray_length.x = (cam_pos.x - float(map_check.x)) * ray_unit_step_size.x;
    }
    else {
        step_i.x = 1;
        ray_length.x = (float(map_check.x + 1) - cam_pos.x) * ray_unit_step_size.x;
    }
    if (dir.y < 0.0) {
        step_i.y = -1;
        ray_length.y = (cam_pos.y - float(map_check.y)) * ray_unit_step_size.y;
    }
    else {
        step_i.y = 1;
        ray_length.y = (float(map_check.y + 1) - cam_pos.y) * ray_unit_step_size.y;
    }
    if (dir.z < 0.0) {
        step_i.z = -1;
        ray_length.z = (cam_pos.z - float(map_check.z)) * ray_unit_step_size.z;
    }
    else {
        step_i.z = 1;
        ray_length.z = (float(map_check.z + 1) - cam_pos.z) * ray_unit_step_size.z;
    }

    bool found = false;
    float max_distance = 800.0;
    float f_distance = 0.0;
    int last_check = 0;
    while (!found && f_distance < max_distance) {

        if (ray_length.x < ray_length.y && ray_length.x < ray_length.z) {
            map_check.x += step_i.x;
            f_distance = ray_length.x;
            ray_length.x += ray_unit_step_size.x;
            last_check = 0;
        }
        else if (ray_length.z < ray_length.x && ray_length.z < ray_length.y) {
            map_check.z += step_i.z;
            f_distance = ray_length.z;
            ray_length.z += ray_unit_step_size.z;
            last_check = 1;
        }
        else {
            map_check.y += step_i.y;
            f_distance = ray_length.y;
            ray_length.y += ray_unit_step_size.y;
            last_check = 2;
        }

        if (map_check.x >= 0 && map_check.x < width && map_check.y >= 0 && map_check.y < height && map_check.z >= 0 && map_check.z < width) {
            
            /*int sv = check_svo_v_pos(map_check);
            if (sv == -1) {
                found = true;
                vec4 modi = vec4(1.0, 1.0, 1.0, 1.0);
                if (last_check < 2) {
                    modi = vec4(0.5, 0.5, 0.5, 1.0);
                    if (last_check < 1) {
                        modi = vec4(0.4, 0.4, 0.4, 1.0);
                    }
                }
                else if (step_i.y > 0) {
                    modi = vec4(0.3, 0.3, 0.3, 1.0);
                }
                f_color = vec4(float(map_check.x) / float(width), float(map_check.y) / float(height), float(map_check.z) / float(width), 1.0) * modi;
            }*/

            vec4 val = texelFetch(sampler3D(world_tex, world_sam), ivec3(map_check.x, map_check.z, map_check.y), 0);

            if (val.a > 0.9) {
                found = true;
                vec4 modi = vec4(1.0, 1.0, 1.0, 1.0);
                if (last_check < 2) {
                    modi = vec4(0.5, 0.5, 0.5, 1.0);
                    if (last_check < 1) {
                        modi = vec4(0.4, 0.4, 0.4, 1.0);
                    }
                }
                else if (step_i.y > 0) {
                    modi = vec4(0.3, 0.3, 0.3, 1.0);
                }
                f_color = val * modi;
                f_color = vec4(float(map_check.x) / float(width), float(map_check.y) / float(height), float(map_check.z) / float(width), 1.0) * modi;
            }
        }
        else {
            //Ray outside the area and not heading into
            if (map_check.x < 0 && step_i.x < 0) {
                break;
            }
            else if (map_check.x >= width && step_i.x > 0) {
                break;
            }

            if (map_check.y < 0 && step_i.y < 0) {
                break;
            }
            else if (map_check.y >= height && step_i.y > 0) {
                break;
            }

            if (map_check.z < 0 && step_i.z < 0) {
                break;
            }
            else if (map_check.z >= width && step_i.z > 0) {
                break;
            }
        }
    }










    if (!found) {
        float sky_val = dot(dir, vec3(0.0, 1.0, 0.0));
        f_color = vec4(max(-sky_val, 0.0), 1.0 * max(sky_val * sky_val, 0.0), 1.0 * max(sky_val, 0.5), 1.0);
    }





    //Shadows
    /*if (found) {

        vec3 intersection_point = cam_pos + dir * (f_distance - 0.001);
        dir = normalize(sun_pos - intersection_point);

        max_distance = 100.0;
        f_distance = 0.0;
        bool found_sun = true;

        ray_unit_step_size = vec3(abs(1.0 / dir.x), abs(1.0 / dir.y), abs(1.0 / dir.z));
        map_check = ivec3(int(intersection_point.x), int(intersection_point.y), int(intersection_point.z));
        ray_length = vec3(0.0, 0.0, 0.0);
        step_i = ivec3(0, 0, 0);

        if (dir.x < 0.0) {
            step_i.x = -1;
            ray_length.x = (intersection_point.x - float(map_check.x)) * ray_unit_step_size.x;
        }
        else {
            step_i.x = 1;
            ray_length.x = (float(map_check.x + 1) - intersection_point.x) * ray_unit_step_size.x;
        }
        if (dir.y < 0.0) {
            step_i.y = -1;
            ray_length.y = (intersection_point.y - float(map_check.y)) * ray_unit_step_size.y;
        }
        else {
            step_i.y = 1;
            ray_length.y = (float(map_check.y + 1) - intersection_point.y) * ray_unit_step_size.y;
        }
        if (dir.z < 0.0) {
            step_i.z = -1;
            ray_length.z = (intersection_point.z - float(map_check.z)) * ray_unit_step_size.z;
        }
        else {
            step_i.z = 1;
            ray_length.z = (float(map_check.z + 1) - intersection_point.z) * ray_unit_step_size.z;
        }

        while (found_sun && f_distance < max_distance) {

            if (ray_length.x < ray_length.y && ray_length.x < ray_length.z) {
                map_check.x += step_i.x;
                f_distance = ray_length.x;
                ray_length.x += ray_unit_step_size.x;
            }
            else if (ray_length.z < ray_length.x && ray_length.z < ray_length.y) {
                map_check.z += step_i.z;
                f_distance = ray_length.z;
                ray_length.z += ray_unit_step_size.z;
            }
            else {
                map_check.y += step_i.y;
                f_distance = ray_length.y;
                ray_length.y += ray_unit_step_size.y;
            }

            if (map_check.x >= 0 && map_check.x < width && -map_check.y >= 0 && -map_check.y < height && map_check.z >= 0 && map_check.z < width) {
                vec4 val = texelFetch(sampler3D(world_tex, world_sam), ivec3(map_check.x, map_check.z, -map_check.y), 0);
                //int val = map[map_check.x + width * (map_check.y + width * map_check.z)];
                if (val.a > 0.5) {

                    f_color = val * vec4(0.1, 0.1, 0.1, 1.0);
                    found_sun = false;
                }
            }
        }

        if (found_sun) {
            f_color.xyz = f_color.xyz * sun_col;
        }
    }*/
}

bool ray_box(vec3 pos, vec3 n_inv, Cube c, out HitInfo hi) {
    
    vec3 tbot = n_inv * (c.mi - pos);
    vec3 ttop = n_inv * (c.ma - pos);

    vec3 tmin = min(ttop, tbot);
    vec3 tmax = min(ttop, tbot);

    vec2 t = max(tmin.xx, tmin.yz);
    float t0 = max(t.x, t.y);
    t = min(tmax.xx, tmax.yz);
    float t1 = min(t.x, t.y);

    hi.tmin = t0;
    hi.tmax = t1;

    return t1 > max(t0, 0.0);
}

Cube box_from_svo(vec3 pos) {

    bool leaf = false;
    bool collide = false;
    int index = 0;
    float offset_x = 0;
    float offset_y = 0;
    float offset_z = 0;
    int leaf_size = 1;
    while (!leaf) {
        if (svo_data[index] == 0) {
            leaf = true;
            leaf_size = svo_data[index + 1];
        }
        else if (svo_data[index] == 2) {
            leaf = true;
            collide = true;
            leaf_size = -1;
        }
        else {
            float size_2 = float(svo_data[index + 1] / 2);
            if (pos.x - offset_x < size_2 && pos.y - offset_y < size_2 && pos.z - offset_z < size_2) {
                index = index + 9;
            }
            else if (pos.x - offset_x >= size_2 && pos.y - offset_y < size_2 && pos.z - offset_z < size_2) {
                index = svo_data[index + 2];
                offset_x += size_2;
            }
            else if (pos.x - offset_x < size_2 && pos.y - offset_y < size_2 && pos.z - offset_z >= size_2) {
                index = svo_data[index + 3];
                offset_z += size_2;
            }
            else if (pos.x - offset_x >= size_2 && pos.y - offset_y < size_2 && pos.z - offset_z >= size_2) {
                index = svo_data[index + 4];
                offset_x += size_2;
                offset_z += size_2;
            }
            else if (pos.x - offset_x < size_2 && pos.y - offset_y >= size_2 && pos.z - offset_z < size_2) {
                index = svo_data[index + 5];
                offset_y += size_2;
            }
            else if (pos.x - offset_x >= size_2 && pos.y - offset_y >= size_2 && pos.z - offset_z < size_2) {
                index = svo_data[index + 6];
                offset_y += size_2;
                offset_x += size_2;
            }
            else if (pos.x - offset_x < size_2 && pos.y - offset_y >= size_2 && pos.z - offset_z >= size_2) {
                index = svo_data[index + 7];
                offset_y += size_2;
                offset_z += size_2;
            }
            else if (pos.x - offset_x >= size_2 && pos.y - offset_y >= size_2 && pos.z - offset_z >= size_2) {
                index = svo_data[index + 8];
                offset_y += size_2;
                offset_x += size_2;
                offset_z += size_2;
            }
        }
    }

    Cube c = Cube(vec3(offset_x, offset_y, offset_z), vec3(offset_x + leaf_size, offset_y + leaf_size, offset_z + leaf_size));

    return c;
}

int check_svo_v_pos(vec3 pos) {
    bool leaf = false;
    bool collide = false;
    int index = 0;
    float offset_x = 0;
    float offset_y = 0;
    float offset_z = 0;
    int leaf_size = 1;
    while (!leaf) {
        if (svo_data[index] == 0) {
            leaf = true;
            leaf_size = svo_data[index + 1];
        }
        else if (svo_data[index] == 2) {
            leaf = true;
            collide = true;
            leaf_size = -1;
        }
        else {
            float size_2 = float(svo_data[index + 1] / 2);
            if (pos.x - offset_x < size_2 && pos.y - offset_y < size_2 && pos.z - offset_z < size_2) {
                index = index + 9;
            }
            else if (pos.x - offset_x >= size_2 && pos.y - offset_y < size_2 && pos.z - offset_z < size_2) {
                index = svo_data[index + 2];
                offset_x += size_2;
            }
            else if (pos.x - offset_x < size_2 && pos.y - offset_y < size_2 && pos.z - offset_z >= size_2) {
                index = svo_data[index + 3];
                offset_z += size_2;
            }
            else if (pos.x - offset_x >= size_2 && pos.y - offset_y < size_2 && pos.z - offset_z >= size_2) {
                index = svo_data[index + 4];
                offset_x += size_2;
                offset_z += size_2;
            }
            else if (pos.x - offset_x < size_2 && pos.y - offset_y >= size_2 && pos.z - offset_z < size_2) {
                index = svo_data[index + 5];
                offset_y += size_2;
            }
            else if (pos.x - offset_x >= size_2 && pos.y - offset_y >= size_2 && pos.z - offset_z < size_2) {
                index = svo_data[index + 6];
                offset_y += size_2;
                offset_x += size_2;
            }
            else if (pos.x - offset_x < size_2 && pos.y - offset_y >= size_2 && pos.z - offset_z >= size_2) {
                index = svo_data[index + 7];
                offset_y += size_2;
                offset_z += size_2;
            }
            else if (pos.x - offset_x >= size_2 && pos.y - offset_y >= size_2 && pos.z - offset_z >= size_2) {
                index = svo_data[index + 8];
                offset_y += size_2;
                offset_x += size_2;
                offset_z += size_2;
            }
        }
    }

    return leaf_size;
}