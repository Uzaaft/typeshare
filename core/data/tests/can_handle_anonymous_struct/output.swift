import Foundation


/// Generated type representing the anonymous struct variant `Us` of the `AutofilledBy` Rust enum
public struct AutofilledByUsInner: Codable {
	/// The UUID for the fill
	public let uuid: String

	public init(uuid: String) {
		self.uuid = uuid
	}
}

/// Generated type representing the anonymous struct variant `SomethingElse` of the `AutofilledBy` Rust enum
public struct AutofilledBySomethingElseInner: Codable {
	/// The UUID for the fill
	public let uuid: String
	/// Some other thing
	public let thing: Int32

	public init(uuid: String, thing: Int32) {
		self.uuid = uuid
		self.thing = thing
	}
}
/// Enum keeping track of who autofilled a field
public enum AutofilledBy: Codable {
	/// This field was autofilled by us
	case us(AutofilledByUsInner)
	/// Something else autofilled this field
	case somethingElse(AutofilledBySomethingElseInner)

	enum CodingKeys: String, CodingKey, Codable {
		case us = "Us",
			somethingElse = "SomethingElse"
	}

	private enum ContainerCodingKeys: String, CodingKey {
		case type, content
	}

	public init(from decoder: Decoder) throws {
		let container = try decoder.container(keyedBy: ContainerCodingKeys.self)
		if let type = try? container.decode(CodingKeys.self, forKey: .type) {
			switch type {
			case .us:
				if let content = try? container.decode(AutofilledByUsInner.self, forKey: .content) {
					self = .us(content)
					return
				}
			case .somethingElse:
				if let content = try? container.decode(AutofilledBySomethingElseInner.self, forKey: .content) {
					self = .somethingElse(content)
					return
				}
			}
		}
		throw DecodingError.typeMismatch(AutofilledBy.self, DecodingError.Context(codingPath: decoder.codingPath, debugDescription: "Wrong type for AutofilledBy"))
	}

	public func encode(to encoder: Encoder) throws {
		var container = encoder.container(keyedBy: ContainerCodingKeys.self)
		switch self {
		case .us(let content):
			try container.encode(CodingKeys.us, forKey: .type)
			try container.encode(content, forKey: .content)
		case .somethingElse(let content):
			try container.encode(CodingKeys.somethingElse, forKey: .type)
			try container.encode(content, forKey: .content)
		}
	}
}


/// Generated type representing the anonymous struct variant `AnonVariant` of the `EnumWithManyVariants` Rust enum
public struct EnumWithManyVariantsAnonVariantInner: Codable {
	public let uuid: String

	public init(uuid: String) {
		self.uuid = uuid
	}
}

/// Generated type representing the anonymous struct variant `AnotherAnonVariant` of the `EnumWithManyVariants` Rust enum
public struct EnumWithManyVariantsAnotherAnonVariantInner: Codable {
	public let uuid: String
	public let thing: Int32

	public init(uuid: String, thing: Int32) {
		self.uuid = uuid
		self.thing = thing
	}
}
/// This is a comment (yareek sameek wuz here)
public enum EnumWithManyVariants: Codable {
	case unitVariant
	case tupleVariantString(String)
	case anonVariant(EnumWithManyVariantsAnonVariantInner)
	case tupleVariantInt(Int32)
	case anotherUnitVariant
	case anotherAnonVariant(EnumWithManyVariantsAnotherAnonVariantInner)

	enum CodingKeys: String, CodingKey, Codable {
		case unitVariant = "UnitVariant",
			tupleVariantString = "TupleVariantString",
			anonVariant = "AnonVariant",
			tupleVariantInt = "TupleVariantInt",
			anotherUnitVariant = "AnotherUnitVariant",
			anotherAnonVariant = "AnotherAnonVariant"
	}

	private enum ContainerCodingKeys: String, CodingKey {
		case type, content
	}

	public init(from decoder: Decoder) throws {
		let container = try decoder.container(keyedBy: ContainerCodingKeys.self)
		if let type = try? container.decode(CodingKeys.self, forKey: .type) {
			switch type {
			case .unitVariant:
				self = .unitVariant
				return
			case .tupleVariantString:
				if let content = try? container.decode(String.self, forKey: .content) {
					self = .tupleVariantString(content)
					return
				}
			case .anonVariant:
				if let content = try? container.decode(EnumWithManyVariantsAnonVariantInner.self, forKey: .content) {
					self = .anonVariant(content)
					return
				}
			case .tupleVariantInt:
				if let content = try? container.decode(Int32.self, forKey: .content) {
					self = .tupleVariantInt(content)
					return
				}
			case .anotherUnitVariant:
				self = .anotherUnitVariant
				return
			case .anotherAnonVariant:
				if let content = try? container.decode(EnumWithManyVariantsAnotherAnonVariantInner.self, forKey: .content) {
					self = .anotherAnonVariant(content)
					return
				}
			}
		}
		throw DecodingError.typeMismatch(EnumWithManyVariants.self, DecodingError.Context(codingPath: decoder.codingPath, debugDescription: "Wrong type for EnumWithManyVariants"))
	}

	public func encode(to encoder: Encoder) throws {
		var container = encoder.container(keyedBy: ContainerCodingKeys.self)
		switch self {
		case .unitVariant:
			try container.encode(CodingKeys.unitVariant, forKey: .type)
		case .tupleVariantString(let content):
			try container.encode(CodingKeys.tupleVariantString, forKey: .type)
			try container.encode(content, forKey: .content)
		case .anonVariant(let content):
			try container.encode(CodingKeys.anonVariant, forKey: .type)
			try container.encode(content, forKey: .content)
		case .tupleVariantInt(let content):
			try container.encode(CodingKeys.tupleVariantInt, forKey: .type)
			try container.encode(content, forKey: .content)
		case .anotherUnitVariant:
			try container.encode(CodingKeys.anotherUnitVariant, forKey: .type)
		case .anotherAnonVariant(let content):
			try container.encode(CodingKeys.anotherAnonVariant, forKey: .type)
			try container.encode(content, forKey: .content)
		}
	}
}
