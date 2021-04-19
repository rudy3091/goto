function goto() {
	case $1 in
		"add") echo "$(go-to $@)" ;;
		*)
			cmd=$(go-to $@)
			if [ $? -eq 1 ]; then
				echo "$cmd"
			else
				echo "$cmd"
				eval ". $cmd"
			fi
			;;
	esac
}
