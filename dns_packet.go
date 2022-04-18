package main

import (
	"encoding/binary"
	"strings"
)

const (
	_QR = 1 << 15
	_AA = 1 << 10
	_TC = 1 << 9
	_RD = 1 << 8
	_RA = 1 << 7
	_Z  = 1 << 6
	_AD = 1 << 5
	_CD = 1 << 4
)

var qtypeLookup = map[string]int{
	"A": 0x0001,
}
var qclassLookup = map[string]int{
	"inet": 0x001,
}

type header struct {
	ID     uint16
	QR     bool
	Opcode int
	AA     bool
	TC     bool
	RD     bool
	RA     bool
	// Z reserved
	RCODE   uint16
	QDCOUNT uint16
	ANCOUNT uint16
	NSCOUNT uint16
	ARCOUNT uint16
}

type question_body struct {
	QNAME  string
	QTYPE  string
	QCLASS string
}

func (q *question_body) marshall() []byte {
	question := make([]byte, 0)
	spl := strings.Split(q.QNAME, ".")
	for _, label := range spl {
		question = append(question, byte(len(label)))
		question = append(question, []byte(label)...)
	}

	hex := qtypeLookup[q.QTYPE]
	typeAndClass := make([]byte, 4)
	binary.BigEndian.PutUint16(typeAndClass, hex)

	//question = append(question,

}

func (h *header) marshall() []byte {
	HEADER := make([]byte, 4*6)
	binary.BigEndian.PutUint16(HEADER[0:4], h.ID)
	binary.BigEndian.PutUint16(HEADER[4:8], h.QDCOUNT)
	binary.BigEndian.PutUint16(HEADER[8:12], h.ANCOUNT)
	binary.BigEndian.PutUint16(HEADER[12:16], h.NSCOUNT)
	binary.BigEndian.PutUint16(HEADER[16:20], h.ARCOUNT)

	_FLAGS := uint16(h.Opcode) << 11
	_FLAGS |= h.RCODE
	if h.QR {
		_FLAGS |= _QR
	}
	if h.AA {
		_FLAGS |= _AA
	}
	if h.TC {
		_FLAGS |= _TC
	}
	if h.RD {
		_FLAGS |= _RD
	}
	if h.RA {
		_FLAGS |= _RA
	}

	FLAGS := make([]byte, 4)
	binary.BigEndian.PutUint16(FLAGS, _FLAGS)
	return HEADER
}
