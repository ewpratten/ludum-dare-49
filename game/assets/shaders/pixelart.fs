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
const vec2 pixelScale = vec2(1.0, 1.0);

void main() {
  // Calculate the distance to merge pixels
  float dx = pixelScale.x * (1.0 / viewport.x);
  float dy = pixelScale.y * (1.0 / viewport.y);

  // Get the base UV coordinate of the pixel
  vec2 baseUV = fragTexCoord;

  // Calculate a UV for this new blocky pixel
  vec2 pixelatedUV = vec2(dx * floor(baseUV.x / dx), dy * floor(baseUV.y / dy));

  // Rebuild the texture with the new UVs
  vec3 tc = texture(texture0, pixelatedUV).rgb;

  // Build the final pixel
  finalColor = vec4(tc, 1.0);
}