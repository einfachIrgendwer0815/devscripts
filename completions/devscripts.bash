#!/bin/bash
# shellcheck disable=SC2207

_devscripts_completions()
{
	COMPREPLY=()

	case ${COMP_WORDS[1]} in
		run)
			case $COMP_CWORD in
			2)
				COMPREPLY=($(compgen -W "$(
					$1 list-scripts | while IFS=: read -r script _; do
						printf "%s\n" "$script"
					done
				)" -- "$2"))
				;;
			3)
				if [[ -z "$2" ]]; then
					COMPREPLY=("--")
				fi
				;;
			esac
			return 0
			;;

		completions)
			case $COMP_CWORD in
			2)
				COMPREPLY=($(compgen -W "
					bash
				" -- "$2"))
				;;
			esac
			return 0
			;;
	esac

	if [[ $COMP_CWORD -gt 1 ]]; then
		return 0
	fi

	if [[ $2 == -* ]]; then
		COMPREPLY=($(compgen -W "
			-h
			--help
			-V
			--version
		" -- "$2"))
		return 0
	fi

	COMPREPLY=($(compgen -W "
		run
	" -- "$2"))

	if [[ ${#COMPREPLY[@]} -eq 0 ]]; then
		COMPREPLY=($(compgen -W "
			list-scripts
			completions
		" -- "$2"))
	fi

	if [[ ${#COMPREPLY[@]} -eq 0 ]]; then
		COMPREPLY=("run")
	fi
	return 0
}

complete -F _devscripts_completions dev
