import Foundation

public struct TypeShareX_OtherType: Codable {
	public init() {}
}

/// This is a comment.
public struct TypeShareX_Person: Codable {
	public let name: String
	public let age: UInt8
	public let extraSpecialFieldOne: Int32
	public let extraSpecialFieldTwo: [String]?
	public let nonStandardDataType: TypeShareX_OtherType
	public let nonStandardDataTypeInArray: [TypeShareX_OtherType]?

	public init(name: String, age: UInt8, extraSpecialFieldOne: Int32, extraSpecialFieldTwo: [String]?, nonStandardDataType: TypeShareX_OtherType, nonStandardDataTypeInArray: [TypeShareX_OtherType]?) {
		self.name = name
		self.age = age
		self.extraSpecialFieldOne = extraSpecialFieldOne
		self.extraSpecialFieldTwo = extraSpecialFieldTwo
		self.nonStandardDataType = nonStandardDataType
		self.nonStandardDataTypeInArray = nonStandardDataTypeInArray
	}
}
