package main

import (
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

const asciiZero = 48
const asciiNine = 57

func main() {
	input, err := os.ReadFile("input.txt")
	if err != nil {
		log.Fatalf("failed to read file: %v", err)
	}

	for i, b := range input {
		// Line feed character
		if b == 10 {
			input = input[:i]
			break
		}
		_, err := DigitToInt(b)
		if err != nil {
			log.Fatalf("unexpected non-digit byte in input: %x", b)
		}
	}
	disk := ParseDiskMap(input)
	CompactDiskMap(disk)

	part1 := Checksum(disk)

	bl := ParseToBlockList(input)
	defragged := Defrag(bl)

	fmt.Printf("Part 1: %d\n", part1)
	fmt.Printf("Part 2: %d\n", defragged.Checksum())
}

func DigitToInt(b byte) (int, error) {
	if b < asciiZero || b > asciiNine {
		return 0, fmt.Errorf("cannot convert non-digit byte to int: %x", b)
	}
	return int(b - asciiZero), nil
}

func MustDigitToInt(b byte) int {
	n, err := DigitToInt(b)
	if err != nil {
		log.Panicf("received error while converting byte to int in call to MustDigitToInt, input: %x", b)
	}
	return n
}

func ParseDiskMap(input []byte) []int {
	totalSlots := 0
	for _, b := range input {
		n := MustDigitToInt(b)
		totalSlots += n
	}
	disk := make([]int, totalSlots)
	head := 0
	for i, b := range input {
		n := MustDigitToInt(b)

		var id int
		if i%2 == 0 {
			// File
			id = i / 2
		} else {
			// Empty space, represented by -1
			id = -1
		}
		for j := 0; j < n; j++ {
			disk[head+j] = id
		}

		head += n
	}

	return disk
}

func CompactDiskMap(disk []int) {
	head := 0
	tail := len(disk) - 1
	for {
		// Advance head to point to the first open slot
		for ; disk[head] != -1; head++ {
		}
		// Decrement tail to point to the first non-open slot
		for ; disk[tail] == -1; tail-- {
		}
		if head >= tail {
			break
		}

		disk[head] = disk[tail]
		disk[tail] = -1
	}
}

func Checksum(disk []int) int {
	checksum := 0
	for idx, id := range disk {
		if id == -1 {
			continue
		}
		checksum += idx * id
	}
	return checksum
}

type BlockList struct {
	id         int
	length     int
	next, prev *BlockList
}

func (bl *BlockList) IsEmpty() bool {
	return bl.id == -1
}

func ParseToBlockList(input []byte) *BlockList {
	if len(input) == 0 {
		return nil
	}

	head := &BlockList{}
	head.id = 0
	head.length = MustDigitToInt(input[0])

	tail := head
	for i, b := range input {
		if i == 0 {
			continue
		}

		var id int
		if i%2 == 0 {
			id = i / 2
		} else {
			id = -1
		}

		length := MustDigitToInt(b)
		if length == 0 {
			continue
		}

		bl := &BlockList{
			id:     id,
			length: length,
			prev:   tail,
		}
		tail.next = bl
		tail = bl
	}

	return head
}

func Defrag(disk *BlockList) *BlockList {
	head := disk
	tail := head
	// Advance tail to end of list
	for ; tail.next != nil; tail = tail.next {
	}

	for {
		// Move "tail" to front until it points at a non-empty block
		for ; tail != nil && tail.IsEmpty(); tail = tail.prev {
		}
		if tail == nil {
			break
		}

		var free *BlockList
		// Move through the list until we find a big enough open slot for "tail"
		ptr := head
		for {
			// If we get to the block we're trying to move then we failed to find an open spot
			if ptr.id == tail.id {
				break
			} else if ptr.IsEmpty() && ptr.length >= tail.length {
				free = ptr
				break
			}
			ptr = ptr.next
		}
		if free == nil {
			tail = tail.prev
			continue
		}

		// Splice "tail" into the beginning of the slot pointed to by "free"
		// First, update pointers at the old location of "tail"
		newTail := &BlockList{
			id:     -1,
			length: tail.length,
			prev:   tail.prev,
			next:   tail.next,
		}
		if newTail.prev != nil {
			newTail.prev.next = newTail
		}
		if newTail.next != nil {
			newTail.next.prev = newTail
		}

		// Now, update pointers where "tail" is getting inserted
		prev := free.prev
		if prev != nil {
			prev.next = tail
		} else {
			// Free block is the first block, so we should update our "head" pointer
			head = tail
		}

		tail.prev = prev
		tail.next = free

		free.prev = tail
		free.length = free.length - tail.length

		tail = newTail
	}

	return head
}

func (bl *BlockList) Render() string {
	var sb strings.Builder
	for head := bl; head != nil; head = head.next {
		var s string
		if head.IsEmpty() {
			s = "."
		} else {
			s = strconv.Itoa(head.id)
		}

		for i := 0; i < head.length; i++ {
			sb.WriteString(s)
		}
	}
	return sb.String()
}

func (bl *BlockList) Checksum() int {
	checksum := 0
	slotsSoFar := 0
	for head := bl; head != nil; head = head.next {
		for i := 0; !head.IsEmpty() && i < head.length; i++ {
			idx := i + slotsSoFar
			checksum += idx * head.id
		}
		slotsSoFar += head.length
	}
	return checksum
}
