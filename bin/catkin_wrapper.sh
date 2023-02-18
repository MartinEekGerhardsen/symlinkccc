
### Wierd way I got it to work

_catkin_wrapper () {
    if (( $# == 0 ))
    then
        \catkin 
    else
        if [[ $1 == "build" ]]
        then 
            \catkin build ${@:2} && symlinkccc
        else 
            \catkin $@
        fi
    fi
}

alias catkin="_catkin_wrapper"