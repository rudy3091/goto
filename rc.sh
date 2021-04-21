function goto() {
	case $1 in
		"add") echo "$(go-to $@)" ;;
		*)
			cmd=$(go-to $@)
			code=$?
			if [ $code -eq 1 ]; then
				echo "$cmd"
			elif [ $code -eq 2 ]; then
				echo "$cmd"
			else
				echo "$cmd"
				eval ". $cmd"
			fi
			;;
	esac
}
