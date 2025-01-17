
fn border(p: vec2<f32>, R2: vec2<f32>) -> f32 {
    let bound: f32 = -sdBox(p - R2 * 0.5, R2 * vec2<f32>(0.5, 0.5));
    let box: f32 = sdBox(Rot(0. * time) * (p - R2 * vec2<f32>(0.5, 0.6)), R2 * vec2<f32>(0.05, 0.01));
    let drain: f32 = -sdBox(p - R2 * vec2<f32>(0.5, 0.7), R2 * vec2<f32>(1.5, 0.5));
    return max(drain, min(bound, box));
} 



fn bN(p: vec2<f32>, R2: vec2<f32>) -> vec3<f32> {
    let dx: vec3<f32> = vec3<f32>(-h, 0., h);
    let idx: vec4<f32> = vec4<f32>(-1. / h, 0., 1. / h, 0.25);
    let r: vec3<f32> = idx.zyw * border(p + dx.zy, R2) + idx.xyw * border(p + dx.xy, R2) + idx.yzw * border(p + dx.yz, R2) + idx.yxw * border(p + dx.yx, R2);
    return vec3<f32>(normalize(r.xy), r.z + 1.0e-4);
} 

fn Simulation(
    ch: texture_storage_2d<rgba32float, read_write>, 
    P: ptr<function, particle>,
    pos: vec2<f32>)  
{
    var F: vec2<f32> = vec2<f32>(0.);
    var avgV: vec3<f32> = vec3<f32>(0.,);

    // var P = *PIn;
    for (var i: i32 = -2; i <= 2; i = i + 1) {
        for (var j: i32 = -2; j <= 2; j = j + 1) {
            let tpos: vec2<f32> = pos + vec2<f32>(f32(i), f32(j));
            // let data: vec4<f32> = texelFetch(ch, Bi(tpos), 0.);

            let data: vec4<f32> = textureLoad(ch, vec2<i32>(tpos)) ;

            var P0: particle = getParticle(data, tpos);
            let dx: vec2<f32> = P0.X - (*P).X;
            let avgP: f32 = 0.5 * P0.M.x * (Pf((*P).M) + Pf(P0.M));
            F = F - (0.5 * G(1. * dx) * avgP * dx);
            avgV = avgV + (P0.M.x * G(1. * dx) * vec3<f32>(P0.V.x, P0.V.y, 1.));
        }
    }

    var avgVxy = avgV.xy;
    avgVxy = avgV.xy / (avgV.z);
    avgV.x = avgVxy.x;
    avgV.y = avgVxy.y;

    F = F + (0. * (*P).M.x * (avgV.xy - (*P).V));
    F = F + ((*P).M.x * vec2<f32>(0., -0.0004));

    if (Mouse.z > 0.) {
        let dm: vec2<f32> = (Mouse.xy - Mouse.zw ) / 10.;
        let d: f32 = distance(Mouse.xy, (*P).X) / 20.;
        F = F + (0.001 * dm * exp(-d * d));

        // let Part = particle(vec2<f32>(0.), vec2<f32>(0.), vec2<f32>(0.));
        // return Part;
    }

    (*P).V = (*P).V + (F * dt / (*P).M.x);

    let N: vec3<f32> = bN((*P).X, R);

    let vdotN: f32 = step(N.z, border_h) * dot(-N.xy, (*P).V);
    (*P).V = (*P).V + (0.5 * (N.xy * vdotN + N.xy * abs(vdotN)));
    (*P).V = (*P).V + (0. * (*P).M.x * N.xy * step(abs(N.z), border_h) * exp(-N.z));

    if (N.z < 0.) {
        (*P).V = vec2<f32>(0.);
    }

    let v: f32 = length((*P).V);
    var dum: f32;
    if (v > 1.) { dum = v; } else { dum = 1.; }
    (*P).V = (*P).V / dum;

    // return P;
}


@compute @workgroup_size(8, 8, 1)
fn update(@builtin(global_invocation_id) invocation_id: vec3<u32>) {
    R = uni.iResolution.xy;
    // let y_inverted_location = vec2<i32>(i32(invocation_id.x), i32(R.y) - i32(invocation_id.y));
    let location = vec2<i32>(i32(invocation_id.x), i32(invocation_id.y));

    var U: vec4<f32>;
    var pos = vec2<f32>(f32(location.x), f32(location.y));

        // R = uni.iResolution.xy;
    time = uni.iTime;
    Mouse = uni.iMouse;
    let p: vec2<i32> = vec2<i32>(pos);
        // let data: vec4<f32> = texel(ch0, pos);
    let data: vec4<f32> = textureLoad(buffer_a, location);

    var P: particle = getParticle(data, pos);

    if (P.M.x != 0.) {
         Simulation(buffer_a, &P, pos);
    }

    let timeMult = 1.0;
    if (length(P.X - R * vec2<f32>(0.55, 0.9)) < 20.) {
        P.X = pos;
        P.V = 0.5 * Dir(-PI * 0.75 + 0.3 * sin(0.4 * 2.0 * timeMult));
        P.M = mix(P.M, vec2<f32>(fluid_rho, 1.), 0.4);
    }

    if (length(P.X - R * vec2<f32>(0.45, 0.9)) < 20.) {
        // P.X = pos;
        // P.V = 0.5 * Dir(-PI * 0.25 + 0.3 * sin(0.3 * timeMult));
        // P.M = mix(P.M, vec2<f32>(fluid_rho, 0.), 0.4);

        P.X = pos;
        P.V = 0.5 * Dir(PI * 0.25 - PI * 0.5 + 0.3 * sin(0.4 * 2.0 * timeMult));
        P.M = mix(P.M, vec2<f32>(fluid_rho, 0.5), 0.4);
    }

    U = saveParticle(P, pos);

    // U = clamp(U, vec4<f32>(-1000000.), vec4<f32>(100000.));
    // U.y = clamp(U.y, -1000000., 100000.);
    textureStore(buffer_b, location, U);


    // let b = vec4<f32>(.5, 0.2, 0.1, 1.0) * -10000.0;
    // textureStore(buffer_b, location, b);
} 

