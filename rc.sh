function goto() {
	case $1 in
		"add") echo "$(go-to $@)" ;;
		*)
			cmd=$(go-to $@)
			code=$?
			if [ $code -eq 1 ]; then
				echo "$cmd"

			elif [ $code -eq 2 ]; then
				idx=2
				state=$(stty -g)
				arr=("${(@s/;/)cmd}")
				echo "\033[?1049h"
				echo "\033[?25l"

				while [ true ]; do
					echo "\033[2J"
					echo "\033[0;0H"

					for i in $(seq 1 ${#arr[@]}); do
						item="${arr[i]}"
						if [ $i -eq $idx ]; then
							echo "> \033[33m$item\033[0m"
						else
							echo "  $item"
						fi
					done

					stty raw
					char=$(dd bs=1 count=1 2> /dev/null)

					case $char in
						"k")
							if [ $idx -gt 2 ]; then
								idx=$((idx - 1))
							fi
							;;
						"j")
							if [ $idx -lt ${#arr[@]} ]; then
								idx=$((idx + 1))
							fi
							;;
						"q")
							stty $state
							break
							;;
						"y")
							stty $state
							eval ". cd ${arr[idx]}"
							break
							;;
					esac

					stty $state
				done
				echo "\033[?25h"
				echo "\033[?1049l"

			else
				echo "$cmd"
				eval ". $cmd"

			fi
			;;
	esac
}
