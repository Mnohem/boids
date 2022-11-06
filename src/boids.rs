use crate::Bird;

impl Bird {
    pub fn flock_rules(&self, boid: &[Bird]) -> [f32; 2] {
        let mut acce = [0.0, 0.0];

        if boid.len() < 1 { return [0.0, 0.0]; }

        let net_force = boid.iter()
            .fold([[0.0; 2]; 3] , |acc, x| {
                let diff = [self.pos[0] - x.pos[0], self.pos[1] - x.pos[1]];
                let distance = dist(self.pos, x.pos);
                [[acc[0][0] + x.vel[0], acc[0][1] + x.vel[1]],//average velocity
                [acc[1][0] + x.pos[0], acc[1][1] + x.pos[1]],//average position
                [acc[2][0] + diff[0]/distance, acc[2][1] + diff[1]/distance]]
                //separation
            })
            .map(|x| x.map(|j| j / boid.len() as f32));

        let clamp_val = 0.3;
        //alignment: go in the direction of the local average velocity
        acce = add(&acce, clamp(
            sub(
                magnitude(net_force[0], 4.0), self.vel), clamp_val));
        //cohesion: go towards local average position
        acce = add(&acce, clamp(
            sub(net_force[1], self.pos), clamp_val));
        //separation: steer away from each other
        add(&acce, clamp(
            sub(
                magnitude(net_force[2], 4.0), self.vel), clamp_val))
    }

    //get the birds around this bird in a range
    //TODO make the range wrap screen borders
    pub fn get_local_boid(&self, birds: &[Bird], range: f32) -> Vec<Bird> {
        birds.iter()
            .filter(|x|  {
                (self.pos != x.pos) && (dist(self.pos, x.pos) < range)
            })
            .map(|x| {
                x.to_owned()
            })
            .collect()
    }
}

pub fn magnitude(v: [f32; 2], mag: f32) -> [f32; 2] {
    let length = dist(v, [0.0; 2]);
    v.map(|x| x * mag / length)
}

fn dist(n1: [f32; 2], n2: [f32; 2]) -> f32 {
    ((n1[0] - n2[0]).powi(2) + (n1[1] - n2[1]).powi(2)).sqrt()
}

fn sub(i: [f32; 2], j: [f32; 2]) -> [f32; 2] {
    [i[0] - j[0], i[1] - j[1]]
}

fn add(i: &[f32; 2], j: [f32; 2]) -> [f32; 2] {
    [i[0] + j[0], i[1] + j[1]]
}

fn clamp(arr: [f32; 2], max: f32) -> [f32; 2] {
    [arr[0].clamp(-max, max), arr[1].clamp(-max, max)]
}

