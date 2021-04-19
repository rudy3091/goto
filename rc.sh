function goto() {
	case $1 in
		"add") echo "$(go-to $@)";;
		*) eval ". $(go-to $@)" ;;
	esac
}
