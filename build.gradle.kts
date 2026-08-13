plugins {
    java

    // For quickly running a local development server.
    id("xyz.jpenilla.run-paper") version "3.0.0"
}

repositories {
    maven {
        name = "papermc"
        url = uri("https://repo.papermc.io/repository/maven-public/")
    }
}

dependencies {
    // Use JUnit Jupiter for testing.
    testImplementation(libs.junit.jupiter)
    testRuntimeOnly("org.junit.platform:junit-platform-launcher")

    // Include the Paper API.
    compileOnly("io.papermc.paper:paper-api:26.2.build.+")
}

// Apply a specific Java toolchain to ease working on different environments.
java {
    toolchain {
        languageVersion = JavaLanguageVersion.of(25)
    }
}

tasks.named<Test>("test") {
    // Use JUnit Platform for unit tests.
    useJUnitPlatform()
}

tasks {
    runServer {
        minecraftVersion("26.2")
    }
}

tasks.jar {
  manifest {
    attributes["paperweight-mappings-namespace"] = "mojang"
  }
}
