package proto

import "encoding/json"

type SomeEnumTypes string
const (
	SomeEnumTypeVariantA SomeEnumTypes = "A"
	SomeEnumTypeVariantC SomeEnumTypes = "C"
)
type SomeEnum struct{ 
	Type SomeEnumTypes `json:"type"`
	content interface{}
}

func (s *SomeEnum) UnmarshalJSON(data []byte) error {
	var enum struct {
		Tag    SomeEnumTypes   `json:"type"`
		Content json.RawMessage `json:"content"`
	}
	if err := json.Unmarshal(data, &enum); err != nil {
		return err
	}

	s.Type = enum.Tag
	switch s.Type {
	case SomeEnumTypeVariantA:
		return nil
	case SomeEnumTypeVariantC:
		var res int
		s.content = &res

	}
	if err := json.Unmarshal(enum.Content, &s.content); err != nil {
		return err
	}

	return nil
}

func (s SomeEnum) MarshalJSON() ([]byte, error) {
    var enum struct {
		Tag    SomeEnumTypes   `json:"type"`
		Content interface{} `json:"content,omitempty"`
    }
    enum.Tag = s.Type
    enum.Content = s.content
    return json.Marshal(enum)
}

func (s SomeEnum) C() int {
	res, _ := s.content.(*int)
	return *res
}

func NewSomeEnumTypeVariantA() SomeEnum {
    return SomeEnum{
        Type: SomeEnumTypeVariantA,
    }
}
func NewSomeEnumTypeVariantC(content int) SomeEnum {
    return SomeEnum{
        Type: SomeEnumTypeVariantC,
        content: &content,
    }
}

