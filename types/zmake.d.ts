
declare module "zmake"{
    /**
     * core version string of semver 2.0
     *
     * Including major.minor.patch only.
     */
    export type VersionCore = `${number}.${number}.${number}`;

    type PreRelease = `-${string}`;
    type BuildVersion = `+${string}`;

    /**
     * full version string of semver 2.0
     *
     * including version core, prerelease and build.
     */
    export type Version = `${VersionCore}${PreRelease | ""}${BuildVersion | ""}`;

    export type GroupId = `${string}`;
    export type ArtifactId = `${GroupId}:${string}`;
    export type QualifiedArtifactId = `${ArtifactId}@${Version}`;

    type Id<Str extends string> = `${QualifiedArtifactId}#${Str}::${string}`;

    /**
     * zmake Id with type `target`
     */
    export type Target = Id<"target">;

    /**
     * zmake Id with type `target_type`
     */
    export type TargetType = Id<"target_type">;

    /**
     * zmake Id with type `architecture`
     */
    export type Architecture = Id<"architecture">;

    /**
     * zmake Id with type `os`
     */
    export type Os = Id<"os">;

    /**
     * zmake Id with type `tool_type`
     */
    export type ToolType = Id<"tool_type">;

    /**
     * zmake Id with type `tool_name`
     */
    export type ToolName = Id<"tool_name">;

    export function requireZMakeVersion(version:Version):void;

    export interface Artifact{
        name : ArtifactId,
        version : Version,
    }

    export type visibility =
        "public" |
        "private"|
        {visibleToDir:[string]} |
        {visibleToFile:[string]} |
        {visibleToArtifact:[ArtifactId]};

    export type transitiveLevel = "public" | "private" | "interface";

    /**
     * project.zmake
     */
    export interface ProjectConfiguration {
        commands:[string],
        scripts:[string],
        build_files:[string]
    }

    export function addCLibrary(artifact:Artifact):Target;
}
