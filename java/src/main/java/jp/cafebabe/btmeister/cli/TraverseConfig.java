package jp.cafebabe.btmeister.cli;

import jp.cafebabe.diranger.Config;
import picocli.CommandLine.Option;

class TraverseConfig {
    @Option(names = { "-a", "--all" }, description = "read hidden directory and files")
    private boolean all = false;

    @Option(names = { "--no-ignore" }, description = "do not respect ignore files (.gitignore).")
    private boolean noIgnore = false;

    @Option(names = { "--follow-symlink" }, description = "follow symbolic links")
    private boolean followSymlinks = false;

    public Config build() {
        return new Config.Builder().respectIgnoreFiles(!noIgnore)
                .skipHiddenFiles(!all)
                .skipSymlinks(!followSymlinks)
                .build();
    }
}
