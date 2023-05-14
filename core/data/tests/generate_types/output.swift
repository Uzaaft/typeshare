import Foundation

public struct CustomType: Codable {
	public init() {}
}

public struct Types: Codable {
	public let s: String
	public let static_s: String
	public let int8: Int8
	public let float: Float
	public let double: Double
	public let array: [String]
	public let fixed_length_array: [String]
	public let dictionary: [String: Int32]
	public let optional_dictionary: [String: Int32]?
	public let custom_type: CustomType

	public init(s: String, static_s: String, int8: Int8, float: Float, double: Double, array: [String], fixed_length_array: [String], dictionary: [String: Int32], optional_dictionary: [String: Int32]?, custom_type: CustomType) {
		self.s = s
		self.static_s = static_s
		self.int8 = int8
		self.float = float
		self.double = double
		self.array = array
		self.fixed_length_array = fixed_length_array
		self.dictionary = dictionary
		self.optional_dictionary = optional_dictionary
		self.custom_type = custom_type
	}
}
