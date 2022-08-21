// License Creative Commons Attribution-NonCommercial-ShareAlike 3.0 Unported License.

// Return random noise in the range [0.0, 1.0], as a function of x.
fn Noise2d( x : vec2<f32> ) -> f32
{
    var xhash : f32 = cos( x.x * 37.0 );
    var yhash : f32 = cos( x.y * 57.0 );
    return fract( 415.92653 * ( xhash + yhash ) );
}

// Convert Noise2d() into a "star field" by stomping everthing below fThreshhold to zero.
fn NoisyStarField( vSamplePos : vec2<f32>, fThreshhold : f32 ) -> f32
{
    var StarVal : f32 = Noise2d( vSamplePos );
    if ( StarVal >= fThreshhold ) {
        StarVal = pow( (StarVal - fThreshhold) / (1.0 - fThreshhold), 6.0 );
    } else {
        StarVal = 0.0;
    }

    return StarVal;
}

// Stabilize NoisyStarField() by only sampling at integer values.
fn StableStarField( vSamplePos : vec2<f32>, fThreshhold : f32 ) -> f32
{
    // Linear interpolation between four samples.
    // Note: This approach has some visual artifacts.
    // There must be a better way to "anti alias" the star field.
    var fractX : f32 = fract( vSamplePos.x );
    var fractY : f32 = fract( vSamplePos.y );
    var floorSample : vec2<f32> = floor( vSamplePos );    
    var v1 : f32 = NoisyStarField( floorSample, fThreshhold );
    var v2 : f32 = NoisyStarField( floorSample + vec2<f32>( 0.0, 1.0 ), fThreshhold );
    var v3 : f32 = NoisyStarField( floorSample + vec2<f32>( 1.0, 0.0 ), fThreshhold );
    var v4 : f32 = NoisyStarField( floorSample + vec2<f32>( 1.0, 1.0 ), fThreshhold );

    var StarVal : f32
                    = v1 * ( 1.0 - fractX ) * ( 1.0 - fractY )
        			+ v2 * ( 1.0 - fractX ) * fractY
        			+ v3 * fractX * ( 1.0 - fractY )
        			+ v4 * fractX * fractY;
	return StarVal;
}

fn toLinear(sRGB: vec4<f32>) -> vec4<f32> {
    let cutoff = vec4<f32>(sRGB < vec4<f32>(0.04045));
    let higher = pow((sRGB + vec4<f32>(0.055)) / vec4<f32>(1.055), vec4<f32>(2.4));
    let lower = sRGB / vec4<f32>(12.92);

    return mix(higher, lower, cutoff);
}

@compute @workgroup_size(8, 8, 1)
fn update(@builtin(global_invocation_id) invocation_id: vec3<u32>) {
	let y_inverted_location = vec2<i32>(i32(invocation_id.x), i32(R.y) - i32(invocation_id.y));
    let location = vec2<i32>(i32(invocation_id.x), i32(invocation_id.y));
	var fragCoord = vec2<f32>(f32(invocation_id.x), f32(invocation_id.y) );

    // Sky Background Color
    var vColor: vec3<f32> = vec3<f32>( 0.1, 0.2, 0.4) * (((uni.iResolution.y - fragCoord.y) / uni.iResolution.y) * 0.2);

    // Note: Choose fThreshhold in the range [0.99, 0.9999].
    // Higher values (i.e., closer to one) yield a sparser starfield.
    let StarFieldThreshhold: f32 = 0.99;

    // Stars with a slow crawl.
    let xRate = 0.0;
    let yRate = 0.0;

    var vSamplePos: vec2<f32> = fragCoord.xy + vec2<f32>( xRate * f32( uni.iFrame ), yRate * f32( uni.iFrame ) );
    var StarVal = StableStarField( vSamplePos, StarFieldThreshhold );
    StarVal = min(StarVal, 0.05);

    vColor += vec3<f32>( StarVal );
	
	var fragColor = vec4<f32>( vColor, 1.0 );
    textureStore(texture, y_inverted_location, toLinear(fragColor));
} 