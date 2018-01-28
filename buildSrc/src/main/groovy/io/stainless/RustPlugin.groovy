package io.stainless.rust

import org.gradle.api.DefaultTask
import org.gradle.api.tasks.TaskAction
import org.gradle.api.Project
import org.gradle.api.Plugin
import org.gradle.api.provider.Property

class RustPlugin implements Plugin<Project> {
    void apply(Project project) {
        def extension = project.extensions.create('rust', RustPluginExtension, project)
        project.task('rustup', type: RustupExec) {
            channel = extension.channel
        }

        project.task('cargoBuild', type: CargoExec) {
            dependsOn('rustup')
            step = "build"
        }

        project.task('cargoTest', type: CargoExec) {
            dependsOn('rustup')
            step = "test"
        }

        project.task('build') {
            dependsOn('cargoBuild')
        }

        project.task('test') {
            dependsOn('cargoTest')
        }
    }
}

class RustupExec extends DefaultTask {
    final Property<String> channel = project.objects.property(String)

    @TaskAction
    void setToolchainCommand() {
        project.exec {
            executable = 'rustup'
            args = ['override', 'set']
            args = args << channel.getOrElse('stable')
        }
    }
}

class CargoExec extends DefaultTask {
    String step

    @TaskAction
    void execCargo() {
        project.exec {
            executable = "cargo"
            args = [ step ]
        }
    }
}