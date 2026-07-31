#!/usr/bin/env bash
# bash completion for flexfetch                          -*- shellscript -*-

_flexfetch() {
	local cur prev
	_init_completion || return

	case "$cur" in
	--*=*) COMPREPLY=() ;;
	-*)
		local opts=(
			--config -c
			--modules -m
			--template -t
			--format -f
			--theme
			--debug
			--gen-config
			--list-modules
			--list-presets
			--benchmark
			--pipe
			--minimal
			--full
			--dev
			--preset
			--export
			--output -o
			--no-gradient
			--no-progress
			--box-style
			--pixel-logo
			--palette-style
			--frame
			--version -V
			--help -h
		)
		COMPREPLY=($(compgen -W "${opts[*]}" -- "$cur"))
		;;
	*)
		if [[ "$prev" == @(config|c|modules|m|template|t|format|f|theme|preset|export|output|o|box-style|palette-style|frame) ]]; then
			case "$prev" in
			format | f) COMPREPLY=($(compgen -W "text json svg html png" -- "$cur")) ;;
			export) COMPREPLY=($(compgen -W "svg html png" -- "$cur")) ;;
			box-style) COMPREPLY=($(compgen -W "rounded sharp double heavy" -- "$cur")) ;;
			palette-style) COMPREPLY=($(compgen -W "gradient solid ansi" -- "$cur")) ;;
			frame) COMPREPLY=($(compgen -W "none single double rounded" -- "$cur")) ;;
			preset) COMPREPLY=($(compgen -W "default minimal full dev server laptop" -- "$cur")) ;;
			*) COMPREPLY=() ;;
			esac
		else
			COMPREPLY=()
		fi
		;;
	esac
}

complete -F _flexfetch flexfetch
