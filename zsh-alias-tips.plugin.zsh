_zsh_alias_tips_rs__PLUGIN_DIR=${0:a:h}

# If zsh-alias-tips-rs is loaded from a symlink, then work out
# where the actual file is located
if [[ -L ${0:a} ]]; then
  _zsh_alias_tips_rs__PLUGIN_DIR=$(readlink ${0:a})
  _zsh_alias_tips_rs__PLUGIN_DIR=${_zsh_alias_tips_rs__PLUGIN_DIR:h}
fi

#export ZSH_PLUGINS_ALIAS_TIPS_TEXT="💡 Alias tip: "
#export ZSH_PLUGINS_ALIAS_TIPS_EXCLUDES="_ c"
#export ZSH_PLUGINS_ALIAS_TIPS_EXPAND=0
#export ZSH_PLUGINS_ALIAS_TIPS_FORCE=0
#export ZSH_PLUGINS_ALIAS_TIPS_REVEAL=0

_zsh_alias_tips_rs__preexec () {
  local CMD=$1
  local CMD_EXPANDED=$2
  if [[ $CMD != $CMD_EXPANDED ]] then # Alias is used
    if [[ ${ZSH_PLUGINS_ALIAS_TIPS_REVEAL-0} == 1 ]] \
    && [[ ${${ZSH_PLUGINS_ALIAS_TIPS_REVEAL_EXCLUDES-()}[(I)$1]} == 0 ]] then # Reveal cmd
        local reveal_text=${ZSH_PLUGINS_ALIAS_TIPS_REVEAL_TEXT-Alias for: }
        local color_dark_gray='\e[1;30m'
        local color_reset='\e[0m'
        echo -e "$color_dark_gray$reveal_text$CMD_EXPANDED $color_reset"
    fi
    if [[ ${ZSH_PLUGINS_ALIAS_TIPS_EXPAND-1} == 0 ]] then
      return 0
    fi
  fi

  local shell_aliases
  shell_aliases=$(alias)

  local shell_functions
  shell_functions=$(functions | grep -E -a '^[a-zA-Z].+ \(\) \{$')

  # Exit code returned from rust binary when we want to force use of aliases.
  local force_exit_code=10
  echo $shell_functions "\n" $shell_aliases | \
    ${_zsh_alias_tips_rs__PLUGIN_DIR}/target/release/zsh-alias-tips-rs $*
  local ret=$?
  if [[ $ret = $force_exit_code ]]; then kill -s INT $$ ; fi
}

autoload -Uz add-zsh-hook
add-zsh-hook preexec _zsh_alias_tips_rs__preexec
