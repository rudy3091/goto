function goto() {
  case $1 in
    "add") echo "$(go-to $@)" ;;
    *)
      go-to $@
      code=$?
      if [ $code -eq 1 ]; then
        echo $cmd
        eval $cmd

      elif [ $code -eq 2 ]; then
        cmd=$(go-to prompt $@)
        echo $cmd
        eval $cmd
      fi
      ;;
  esac
}
