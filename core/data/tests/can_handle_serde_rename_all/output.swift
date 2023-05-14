import Foundation

/// This is a Person struct with camelCase rename
public struct Person: Codable {
	public let firstName: String
	public let lastName: String
	public let age: UInt8
	public let extraSpecialField1: Int32
	public let extraSpecialField2: [String]?

	public init(firstName: String, lastName: String, age: UInt8, extraSpecialField1: Int32, extraSpecialField2: [String]?) {
		self.firstName = firstName
		self.lastName = lastName
		self.age = age
		self.extraSpecialField1 = extraSpecialField1
		self.extraSpecialField2 = extraSpecialField2
	}
}

/// This is a Person2 struct with UPPERCASE rename
public struct Person2: Codable {
	public let FIRST_NAME: String
	public let LAST_NAME: String
	public let AGE: UInt8

	public init(FIRST_NAME: String, LAST_NAME: String, AGE: UInt8) {
		self.FIRST_NAME = FIRST_NAME
		self.LAST_NAME = LAST_NAME
		self.AGE = AGE
	}
}
