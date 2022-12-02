#!/bin/bash

FILENAME=$1
LINES=$(cat $FILENAME)

# given R=0, P=1 and S=2
# (RD, RL, RW, PW, PD, PL, SL, SW, SD)
ROUND_SCORE_LOOKUP=(4 1 7 8 5 2 3 9 6)
declare -a SCORE_ARRAY

MY_HAND_INDEX=0
COUNTER=0
ELF_HAND_INDEX=0
for LINE in $LINES
do
  # echo "$LINE"
  IS_MY_HAND=$(($COUNTER % 2 ))
  # we are looking at the hand we would play
  if [ $IS_MY_HAND -eq 1 ]; then 
    # rock? -> score = 1
    if [ "$LINE" = "X" ]; then
      MY_HAND_INDEX=0
    # paper? -> score = 2
    elif [ "$LINE" = "Y" ]; then
      MY_HAND_INDEX=1
    # scissors? -> score = 3
    else
      MY_HAND_INDEX=2
    fi

    SCORE_ARRAY_INDEX=$(($COUNTER - 1))
    SCORE_ARRAY_INDEX=$(($SCORE_ARRAY_INDEX / 2))

    SCORE_TO_ADD=$((ROUND_SCORE_LOOKUP[ ($MY_HAND_INDEX * 3) + $ELF_HAND_INDEX ] )) 

    SCORE_ARRAY[$SCORE_ARRAY_INDEX]="$SCORE_TO_ADD"
   
  # we are looking at the elf's hand
  else
    if [ "$LINE" = "A" ]; then
      ELF_HAND_INDEX=0
    # paper? -> score = 2
    elif [ "$LINE" = "B" ]; then
      ELF_HAND_INDEX=1
    # scissors? -> score = 3
    else
      ELF_HAND_INDEX=2
    fi
  fi
  COUNTER=$(($COUNTER + 1))
done

TOTAL_SCORE=0
for ROUND_SCORE in ${SCORE_ARRAY[@]}
do
  TOTAL_SCORE=$(($TOTAL_SCORE + $ROUND_SCORE))
done

echo "$TOTAL_SCORE"
