function goto() {
  case $1 in
    "add") echo "$(go-to $@)" ;;
    *)
      go-to $@
      code=$?
      if [ $code -eq 0 ]; then # matches one
        cmd=$(go-to $@)
        eval $cmd

      elif [ $code -eq 2 ]; then # matches multiple
        cmd=$(go-to prompt $@)
        code=$?
        echo $cmd

        if [ $code -eq 0 ]; then
          eval $cmd
        fi
      fi
      ;;
  esac
}
