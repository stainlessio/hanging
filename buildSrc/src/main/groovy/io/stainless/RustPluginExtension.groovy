package io.stainless.rust

import org.gradle.api.DefaultTask
import org.gradle.api.tasks.TaskAction
import org.gradle.api.Project
import org.gradle.api.Plugin
import org.gradle.api.provider.Property

class RustPluginExtension {
    final Property<String> channel
    final Property<String> version

    RustPluginExtension(Project project) {
        channel = project.objects.property(String)
        channel.set('stable')
        version = project.objects.property(String)
        version.set('latest')
    }
}