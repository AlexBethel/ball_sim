#version 330 core

in VS_OUTPUT {
  vec3 Color;
  vec2 UV;
} IN;

out vec4 Color;

void main() {
  if (IN.UV.x * IN.UV.x + IN.UV.y * IN.UV.y < 1.0)
    Color = vec4(IN.Color, 1.0f);
  else
    Color = vec4(0.0, 0.0, 0.0, 0.0);
}
