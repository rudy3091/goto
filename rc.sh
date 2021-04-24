function goto_restore() {
	stty $2
	eval ". cd $1"
}

function goto() {
	case $1 in
		"add") echo "$(go-to $@)" ;;
		*)
			cmd=$(go-to $@)
			code=$?
			if [ $code -eq 1 ]; then
				echo "$cmd"

			elif [ $code -eq 2 ]; then
				idx=0
				state=$(stty -g)
				arr=("${(@s/;/)cmd}")
				echo "\033[?1049h"
				echo "\033[?25l"

				while [ true ]; do
					echo "\033[2J"
					echo "\033[0;0H"
					echo "${arr[1]}"

					for i in $(seq 2 11); do
						item="${arr[i + idx * 10]}"
						if ! [ -z $item ]; then
							echo "\033[33m[$((i - 2))]\033[0m: $item\033[0m"
						fi
					done
					echo "\033[2;38H\033[1m[ $((idx + 1)) / 2 ]\033[0m"

					stty raw
					char=$(dd bs=1 count=1 2> /dev/null)

					case $char in
						"k")
							if [ $idx -gt 0 ]; then
								idx=$((idx - 1))
							fi
							;;
						"j")
							if [ $idx -lt $((${#arr[@]} / 10)) ]; then
								idx=$((idx + 1))
							fi
							;;
						"q")
							stty $state
							break
							;;
						*)
							if [ $((idx * 10 + char + 2)) -gt $((${#arr[@]})) ]; then
								stty $state
								continue
							fi

							if [ $char -eq 0 ]; then
								goto_restore ${arr[idx * 10 + 2]} $state; break
							elif [ $char -eq 1 ]; then
								goto_restore ${arr[idx * 10 + 3]} $state; break
							elif [ $char -eq 2 ]; then
								goto_restore ${arr[idx * 10 + 4]} $state; break
							elif [ $char -eq 3 ]; then
								goto_restore ${arr[idx * 10 + 5]} $state; break
							elif [ $char -eq 4 ]; then
								goto_restore ${arr[idx * 10 + 6]} $state; break
							elif [ $char -eq 5 ]; then
								goto_restore ${arr[idx * 10 + 7]} $state; break
							elif [ $char -eq 6 ]; then
								goto_restore ${arr[idx * 10 + 8]} $state; break
							elif [ $char -eq 7 ]; then
								goto_restore ${arr[idx * 10 + 9]} $state; break
							elif [ $char -eq 8 ]; then
								goto_restore ${arr[idx * 10 + 10]} $state; break
							elif [ $char -eq 9 ]; then
								goto_restore ${arr[idx * 10 + 11]} $state; break
							else
								stty $state
							fi
							;;
					esac

					stty $state
				done
				echo "\033[?25h"
				echo "\033[?1049l"
				echo "\033[33mChanging to ${arr[idx * 10 + $((char + 2))]}\033[0m"

			else
				echo "$cmd"
				eval ". $cmd"

			fi
			;;
	esac
}
