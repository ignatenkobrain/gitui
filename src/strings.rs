pub static TITLE_STATUS: &str = "Unstaged Changes [1]";
pub static TITLE_DIFF: &str = "Diff: ";
pub static TITLE_INDEX: &str = "Staged Changes [2]";

pub static TAB_STATUS: &str = "Status";
pub static TAB_STASHING: &str = "Stashing";
pub static TAB_LOG: &str = "Log";
pub static TAB_DIVIDER: &str = "  |  ";

pub static CMD_SPLITTER: &str = " ";

pub static MSG_TITLE_ERROR: &str = "Error";
pub static COMMIT_TITLE: &str = "Commit";
pub static COMMIT_MSG: &str = "type commit message..";
pub static STASH_POPUP_TITLE: &str = "Stash";
pub static STASH_POPUP_MSG: &str = "type name (optional)";
pub static CONFIRM_TITLE_RESET: &str = "Reset";
pub static CONFIRM_TITLE_STASHDROP: &str = "Drop";
pub static CONFIRM_MSG_RESET: &str = "confirm file reset?";
pub static CONFIRM_MSG_STASHDROP: &str = "confirm stash drop?";

pub static LOG_TITLE: &str = "Commit";
pub static STASHLIST_TITLE: &str = "Stashes";

pub static HELP_TITLE: &str = "Help: all commands";

pub static STASHING_FILES_TITLE: &str = "Files to Stash";
pub static STASHING_OPTIONS_TITLE: &str = "Options";

pub mod commands {
    use crate::components::CommandText;

    static CMD_GROUP_GENERAL: &str = "General";
    static CMD_GROUP_DIFF: &str = "Diff";
    static CMD_GROUP_CHANGES: &str = "Changes";
    static CMD_GROUP_COMMIT: &str = "Commit";
    static CMD_GROUP_STASHING: &str = "Stashing";
    static CMD_GROUP_STASHES: &str = "Stashes";

    ///
    pub static TOGGLE_TABS: CommandText = CommandText::new(
        "Tabs [tab]",
        "switch top level tabs",
        CMD_GROUP_GENERAL,
    );
    ///
    pub static HELP_OPEN: CommandText = CommandText::new(
        "Help [h]",
        "open this help screen",
        CMD_GROUP_GENERAL,
    );
    ///
    pub static NAVIGATE_TREE: CommandText = CommandText::new(
        "Nav [\u{2190}\u{2191}\u{2192}\u{2193}]",
        "navigate tree view",
        CMD_GROUP_GENERAL,
    );
    ///
    pub static SCROLL: CommandText = CommandText::new(
        "Scroll [\u{2191}\u{2193}]",
        "scroll up or down in focused view",
        CMD_GROUP_GENERAL,
    );
    ///
    pub static DIFF_HOME_END: CommandText = CommandText::new(
        "Jump up/down [home,end,shift+up,shift+down]",
        "scroll to top or bottom of diff",
        CMD_GROUP_DIFF,
    );
    ///
    pub static DIFF_HUNK_ADD: CommandText = CommandText::new(
        "Add hunk [enter]",
        "adds selected hunk to stage",
        CMD_GROUP_DIFF,
    );
    ///
    pub static DIFF_HUNK_REMOVE: CommandText = CommandText::new(
        "Remove hunk [enter]",
        "removes selected hunk from stage",
        CMD_GROUP_DIFF,
    );
    ///
    pub static CLOSE_POPUP: CommandText = CommandText::new(
        "Close [esc]",
        "close overlay (e.g commit, help)",
        CMD_GROUP_GENERAL,
    );
    ///
    pub static CLOSE_MSG: CommandText = CommandText::new(
        "Close [enter]",
        "close msg popup (e.g msg)",
        CMD_GROUP_GENERAL,
    )
    .hide_help();
    ///
    pub static SELECT_STAGING: CommandText = CommandText::new(
        "Focus Stage [2]",
        "focus/select staging area",
        CMD_GROUP_GENERAL,
    );
    ///
    pub static SELECT_STATUS: CommandText = CommandText::new(
        "Focus Files [1,2]",
        "focus/select file tree of staged or unstaged files",
        CMD_GROUP_GENERAL,
    );
    ///
    pub static SELECT_UNSTAGED: CommandText = CommandText::new(
        "Focus Unstaged [1]",
        "focus/select unstaged area",
        CMD_GROUP_GENERAL,
    );
    ///
    pub static COMMIT_OPEN: CommandText = CommandText::new(
        "Commit [c]",
        "open commit popup (available in non-empty stage)",
        CMD_GROUP_COMMIT,
    );
    ///
    pub static COMMIT_ENTER: CommandText = CommandText::new(
        "Commit [enter]",
        "commit (available when commit message is non-empty)",
        CMD_GROUP_COMMIT,
    );
    ///
    pub static STAGE_ITEM: CommandText = CommandText::new(
        "Stage Item [enter]",
        "stage currently selected file or entire path",
        CMD_GROUP_CHANGES,
    );
    ///
    pub static UNSTAGE_ITEM: CommandText = CommandText::new(
        "Unstage Item [enter]",
        "unstage currently selected file or entire path",
        CMD_GROUP_CHANGES,
    );
    ///
    pub static RESET_ITEM: CommandText = CommandText::new(
        "Reset Item [D]",
        "revert changes in selected file or entire path",
        CMD_GROUP_CHANGES,
    );

    ///
    pub static STATUS_FOCUS_LEFT: CommandText = CommandText::new(
        "Back [\u{2190}]", //←
        "view staged changes",
        CMD_GROUP_GENERAL,
    );
    ///
    pub static STATUS_FOCUS_RIGHT: CommandText = CommandText::new(
        "Diff [\u{2192}]", //→
        "inspect file diff",
        CMD_GROUP_CHANGES,
    );
    ///
    pub static QUIT: CommandText = CommandText::new(
        "Quit [ctrl+c]",
        "quit gitui application",
        CMD_GROUP_GENERAL,
    );
    ///
    pub static RESET_CONFIRM: CommandText = CommandText::new(
        "Confirm [enter]",
        "resets the file in question",
        CMD_GROUP_GENERAL,
    );

    ///
    pub static STASHING_SAVE: CommandText = CommandText::new(
        "Save [s]",
        "opens stash name input popup",
        CMD_GROUP_STASHING,
    );
    ///
    pub static STASHING_TOGGLE_INDEXED: CommandText =
        CommandText::new(
            "Toggle Staged [i]",
            "toggle including staged files into stash",
            CMD_GROUP_STASHING,
        );
    ///
    pub static STASHING_TOGGLE_UNTRACKED: CommandText =
        CommandText::new(
            "Toggle Untracked [u]",
            "toggle including untracked files into stash",
            CMD_GROUP_STASHING,
        );
    ///
    pub static STASHING_CONFIRM_MSG: CommandText = CommandText::new(
        "Stash [enter]",
        "save files to stash",
        CMD_GROUP_STASHING,
    );
    ///
    pub static STASHLIST_APPLY: CommandText = CommandText::new(
        "Apply [enter]",
        "apply selected stash",
        CMD_GROUP_STASHES,
    );
    ///
    pub static STASHLIST_DROP: CommandText = CommandText::new(
        "Drop [D]",
        "drop selected stash",
        CMD_GROUP_STASHES,
    );
}
