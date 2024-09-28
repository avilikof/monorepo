package adventofcode2023

import (
	"os"
	"testing"
)

func TestNewCalibrationHandler(t *testing.T) {
	doc := NewCalibrationHandler([]string{})
	if doc == nil {
		t.Error("Expected non-nil CalibrationDocument")
	}
}

// func TestPutFileContent(t *testing.T) {
// 	doc := NewCalibrationDocument()
// 	content := []byte("test content")
// 	_err := doc.PutFileContent(content)
// 	if _err != nil {
// 		t.Errorf("PutFileContent returned error: %v", _err)
// 	}
// 	if string(doc.fileContent) != string(content) {
// 		t.Errorf("Expected %v, got %v", content, doc.fileContent)
// 	}
// }

// func TestPutLines(t *testing.T) {
// 	doc := NewCalibrationDocument()
// 	lines := []string{"line1", "line2"}
// 	_err := doc.PutLines(lines)
// 	if _err != nil {
// 		t.Errorf("PutLines returned error: %v", _err)
// 	}
// 	if len(doc.lines) != len(lines) {
// 		t.Errorf("Expected %v, got %v", lines, doc.lines)
// 	}
// }

// func TestGetFileContentString(t *testing.T) {
// 	doc := NewCalibrationDocument()
// 	_, _err := doc.GetFileContentString()
// 	if _err == nil {
// 		t.Error("Expected error, got nil")
// 	}

// 	content := []byte("test content")
// 	doc.PutFileContent(content)
// 	str, _err := doc.GetFileContentString()
// 	if _err != nil {
// 		t.Errorf("GetFileContentString returned error: %v", _err)
// 	}
// 	if str != string(content) {
// 		t.Errorf("Expected %v, got %v", string(content), str)
// 	}
// }

func TestReadFile(t *testing.T) {
	const testFileName = "testfile.txt"
	testContent := []byte("test content")
	_err := os.WriteFile(testFileName, testContent, 0644)
	if _err != nil {
		t.Fatalf("Failed to create test file: %v", _err)
	}
	defer os.Remove(testFileName)

	content, _err := ReadFile(testFileName)
	if _err != nil {
		t.Errorf("ReadFile returned error: %v", _err)
	}
	if string(content) != string(testContent) {
		t.Errorf("Expected %v, got %v", string(testContent), string(content))
	}
}

// func TestSplitToLines(t *testing.T) {
// 	data := "line1\nline2\nline3\n"
// 	lines, _err := SplitToLines(data)
// 	if _err != nil {
// 		t.Errorf("SplitToLines returned error: %v", _err)
// 	}
// 	expected := []string{"line1", "line2", "line3"}
// 	if len(lines) != len(expected) {
// 		t.Errorf("Expected %v, got %v", expected, lines)
// 	}
// 	for i, line := range lines {
// 		if line != expected[i] {
// 			t.Errorf("Expected %v, got %v", expected[i], line)
// 		}
// 	}
// }

// func TestRemoveLastBlankLine(t *testing.T) {
// 	lines := []string{"line1", "line2", ""}
// 	expected := []string{"line1", "line2"}
// 	result := RemoveLastBlankLine(lines)
// 	if len(result) != len(expected) {
// 		t.Errorf("Expected %v, got %v", expected, result)
// 	}
// 	for i, line := range result {
// 		if line != expected[i] {
// 			t.Errorf("Expected %v, got %v", expected[i], line)
// 		}
// 	}
// }
