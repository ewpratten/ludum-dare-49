/**
 * This shader is the last piece of the graphics pipeline. EVERYTHING is passed through it.
 */

#version 330

// Fragment texture UV coordinate
in vec2 fragTexCoord;

// Whole input texture
uniform sampler2D texture0;

// Viewport size
uniform vec2 viewport;

// Output fragment color
out vec4 finalColor;

// Pixel scaling factor (pixels per pixel)
uniform vec2 pixelScale;

// CRT effect parameters
uniform float warpFactor;
uniform float scanlineDarkness;

void main() {
  // Calculate the distance to merge pixels
  float dx = pixelScale.x * (1.0 / viewport.x);
  float dy = pixelScale.y * (1.0 / viewport.y);

  // Calculate the squared distance from the center
  vec2 dist_center_sq = abs(0.5 - fragTexCoord);
  dist_center_sq *= dist_center_sq;

  // Get the base UV coordinate of the pixel
  vec2 baseUV = fragTexCoord;

  // Calculate a UV for this new blocky pixel
  vec2 pixelatedUV = vec2(dx * floor(baseUV.x / dx), dy * floor(baseUV.y / dy));

  // Warp the UVs of the pixelated texture
  vec2 warpedUV = pixelatedUV;
  warpedUV.x -= 0.5; warpedUV.x *= 1.0+(dist_center_sq.y * (0.3 * warpFactor)); warpedUV.x += 0.5;
  warpedUV.y -= 0.5; warpedUV.y *= 1.0+(dist_center_sq.x * (0.4 * warpFactor)); warpedUV.y += 0.5;

  // If the UV is outside the texture, return black
  if (warpedUV.x < 0.0 || warpedUV.x > 1.0 || warpedUV.y < 0.0 || warpedUV.y > 1.0) {
    finalColor = vec4(0.0, 0.0, 0.0, 1.0);
    return;
  }

  // Determine factor of if we are rendering on a scanline
  float scanlineFactor = abs(sin(fragTexCoord.y * viewport.y) * 0.5 * scanlineDarkness);

  // Build the final pixel
  finalColor = vec4(mix(texture(texture0, warpedUV).rgb, vec3(0.0), scanlineFactor), 1.0);
}
