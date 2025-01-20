// Exercise 3:
//
// Create two shader programs where the second program uses a different fragment
// shader that outputs the color yellow; draw both triangles again where one outputs the color yellow
//
// Solution: https://learnopengl.com/code_viewer_gh.php?code=src/1.getting_started/2.5.hello_triangle_exercise3/hello_triangle_exercise3.cpp

#include <stdio.h>
#include <iostream>

#define GL_SILENCE_DEPRECATION

// Without this gl.h gets included instead of gl3.h
#define GLFW_INCLUDE_NONE
#include <GLFW/glfw3.h>

// For includes related to OpenGL, make sure their are included after glfw3.h
#include <OpenGL/gl3.h>

void errorCallback(int error, const char *description)
{
    fputs(description, stderr);
}

void keyCallback(GLFWwindow *window, int key, int scancode, int action, int mods)
{
    if (key == GLFW_KEY_ESCAPE && action == GLFW_PRESS)
        glfwSetWindowShouldClose(window, GLFW_TRUE);
}

void frameBufferResizeCallback(GLFWwindow *window, int width, int height)
{
    glViewport(0, 0, width, height);
}

const unsigned int SCR_WIDTH = 640;
const unsigned int SCR_HEIGHT = 480;

const char *vertexShaderSource = "#version 410 core\n"
                                 "layout (location = 0) in vec3 aPos;\n"
                                 "void main()\n"
                                 "{\n"
                                 "   gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);\n"
                                 "}\0";

const char *fragmentShader1Source = "#version 410 core\n"
                                    "out vec4 FragColor;\n"
                                    "void main()\n"
                                    "{\n"
                                    "   FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);\n"
                                    "}\0";

const char *fragmentShader2Source = "#version 410 core\n"
                                    "out vec4 FragColor;\n"
                                    "void main()\n"
                                    "{\n"
                                    "   FragColor = vec4(1.0f, 1.0f, 0.0f, 1.0f);\n"
                                    "}\0";

int main(void)
{
    // glfw: initialize and configure
    // ------------------------------
    // Initialize the library
    if (!glfwInit())
        return -1;
    glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 4);
    glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 1);

    // Without these two hints, nothing above OpenGL version 2.1 is supported
    glfwWindowHint(GLFW_OPENGL_FORWARD_COMPAT, GLFW_TRUE);
    glfwWindowHint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE);

    // Set callback for errors
    glfwSetErrorCallback(errorCallback);

    // glfw window creation
    // --------------------
    // Create a windowed mode window and its OpenGL context
    GLFWwindow *window = glfwCreateWindow(SCR_WIDTH, SCR_HEIGHT, "Rectangle", NULL, NULL);
    if (!window)
    {
        std::cout << "Failed to create GLFW window" << std::endl;
        glfwTerminate();
        return -1;
    }

    // Make the window's context current
    glfwMakeContextCurrent(window);

    // Set callback for window
    glfwSetKeyCallback(window, keyCallback);

    // Set callback fro framebuffer
    glfwSetFramebufferSizeCallback(window, frameBufferResizeCallback);

    // Used to avoid screen tearing
    glfwSwapInterval(1);

    // build and compile our shader program
    // ------------------------------------
    int success;
    char infoLog[512];

    // Vertex shader
    unsigned int vertexShader = glCreateShader(GL_VERTEX_SHADER);
    glShaderSource(vertexShader, 1, &vertexShaderSource, NULL);
    glCompileShader(vertexShader);
    glGetShaderiv(vertexShader, GL_COMPILE_STATUS, &success);
    if (!success)
    {
        glGetShaderInfoLog(vertexShader, 512, NULL, infoLog);
        std::cout << "ERROR::SHADER::VERTEX::COMPILATION_FAILED\n"
                  << infoLog << std::endl;
    }

    // Fragment shader
    unsigned int fragmentShaderOrange = glCreateShader(GL_FRAGMENT_SHADER);
    glShaderSource(fragmentShaderOrange, 1, &fragmentShader1Source, NULL);
    glCompileShader(fragmentShaderOrange);
    glGetShaderiv(fragmentShaderOrange, GL_COMPILE_STATUS, &success);
    if (!success)
    {
        glGetShaderInfoLog(vertexShader, 512, NULL, infoLog);
        std::cout << "ERROR::SHADER::ORANGE_FRAGMENT::COMPILATION_FAILED\n"
                  << infoLog << std::endl;
    }

    unsigned int fragmentShaderYellow = glCreateShader(GL_FRAGMENT_SHADER);
    glShaderSource(fragmentShaderYellow, 1, &fragmentShader2Source, NULL);
    glCompileShader(fragmentShaderYellow);
    glGetShaderiv(fragmentShaderYellow, GL_COMPILE_STATUS, &success);
    if (!success)
    {
        glGetShaderInfoLog(vertexShader, 512, NULL, infoLog);
        std::cout << "ERROR::SHADER::YELLOW_FRAGMENT::COMPILATION_FAILED\n"
                  << infoLog << std::endl;
    }

    // Shader program
    unsigned int shaderProgramOrange = glCreateProgram();
    glAttachShader(shaderProgramOrange, vertexShader);
    glAttachShader(shaderProgramOrange, fragmentShaderOrange);
    glLinkProgram(shaderProgramOrange);
    glGetProgramiv(shaderProgramOrange, GL_LINK_STATUS, &success);
    if (!success)
    {
        glGetProgramInfoLog(shaderProgramOrange, 512, NULL, infoLog);
        std::cout << "ERROR::SHADER::PROGRAM::LINK_FAILED\n"
                  << infoLog << std::endl;
    }

    unsigned int shaderProgramYellow = glCreateProgram();
    glAttachShader(shaderProgramYellow, vertexShader);
    glAttachShader(shaderProgramYellow, fragmentShaderYellow);
    glLinkProgram(shaderProgramYellow);
    glGetProgramiv(shaderProgramYellow, GL_LINK_STATUS, &success);
    if (!success)
    {
        glGetProgramInfoLog(shaderProgramYellow, 512, NULL, infoLog);
        std::cout << "ERROR::SHADER::PROGRAM::LINK_FAILED\n"
                  << infoLog << std::endl;
    }

    glDeleteShader(vertexShader);
    glDeleteShader(fragmentShaderOrange);
    glDeleteShader(fragmentShaderYellow);

    // set up vertex data (and buffer(s)) and configure vertex attributes
    // ------------------------------------------------------------------
    float firstTriangle[] = {
        -0.5f, 0.2f, 0.0f, // left
        0.5f, 0.2f, 0.0f,  // right bottom
        0.5f, 0.8f, 0.0f,  // right top
    };

    float secondTriangle[] = {
        -0.5f, -0.2f, 0.0f, // left top
        -0.5f, -0.8f, 0.0f, // left bottom
        0.5f, -0.2f, 0.0f,  // right
    };

    unsigned int VAOs[2], VBOs[2];
    glGenVertexArrays(2, VAOs);
    glGenBuffers(2, VBOs);

    glBindVertexArray(VAOs[0]);
    glBindBuffer(GL_ARRAY_BUFFER, VBOs[0]);
    glBufferData(GL_ARRAY_BUFFER, sizeof(firstTriangle), firstTriangle, GL_STATIC_DRAW);
    glVertexAttribPointer(0, 3, GL_FLOAT, GL_FALSE, 3 * sizeof(float), (void *)0);
    glEnableVertexAttribArray(0);

    glBindVertexArray(VAOs[1]);
    glBindBuffer(GL_ARRAY_BUFFER, VBOs[1]);
    glBufferData(GL_ARRAY_BUFFER, sizeof(secondTriangle), secondTriangle, GL_STATIC_DRAW);
    glVertexAttribPointer(0, 3, GL_FLOAT, GL_FALSE, 3 * sizeof(float), (void *)0);
    glEnableVertexAttribArray(0);

    glBindBuffer(GL_ARRAY_BUFFER, 0);
    glClearColor(0.2f, 0.3f, 0.3f, 1.0f);

    while (!glfwWindowShouldClose(window))
    {
        glClear(GL_COLOR_BUFFER_BIT);

        glUseProgram(shaderProgramOrange);
        glBindVertexArray(VAOs[0]);
        glDrawArrays(GL_TRIANGLES, 0, 3);

        glUseProgram(shaderProgramYellow);
        glBindVertexArray(VAOs[1]);
        glDrawArrays(GL_TRIANGLES, 0, 3);

        glfwSwapBuffers(window);
        glfwPollEvents();
    }

    // optional: de-allocate all resources once they've outlived their purpose:
    // ------------------------------------------------------------------------
    glDeleteVertexArrays(2, VAOs);
    glDeleteBuffers(2, VBOs);
    glDeleteProgram(shaderProgramOrange);
    glDeleteProgram(shaderProgramYellow);

    // glfw: terminate
    // ------------------------------------------------------------------
    glfwDestroyWindow(window);
    glfwTerminate();
    return 0;
}