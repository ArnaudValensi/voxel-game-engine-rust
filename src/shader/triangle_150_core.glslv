#version 150 core

in vec3 a_Pos;
in vec3 a_Color;
out vec4 v_Color;

layout (std140)
uniform Locals {
	mat4 u_Model;
	mat4 u_View;
	mat4 u_Proj;
};

void main() {
    v_Color = vec4(a_Color, 1.0);
    gl_Position = u_Proj * u_View * u_Model * vec4(a_Pos, 1.0);
}
