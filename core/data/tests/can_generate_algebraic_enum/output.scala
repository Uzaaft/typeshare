package com.agilebits

package object onepassword {

type UByte = Byte
type UShort = Short
type UInt = Int
type ULong = Int

}
package onepassword {

// Struct comment
class ItemDetailsFieldValue extends Serializable

// Enum comment
sealed trait AdvancedColors {
	def serialName: String
}
object AdvancedColors {
	// This is a case comment
	case class String(content: String) extends AdvancedColors {
		val serialName: String = "String"
	}
	case class Number(content: Int) extends AdvancedColors {
		val serialName: String = "Number"
	}
	case class UnsignedNumber(content: UInt) extends AdvancedColors {
		val serialName: String = "UnsignedNumber"
	}
	case class NumberArray(content: Vector[Int]) extends AdvancedColors {
		val serialName: String = "NumberArray"
	}
	// Comment on the last element
	case class ReallyCoolType(content: ItemDetailsFieldValue) extends AdvancedColors {
		val serialName: String = "ReallyCoolType"
	}
}

sealed trait AdvancedColors2 {
	def serialName: String
}
object AdvancedColors2 {
	// This is a case comment
	case class String(content: String) extends AdvancedColors2 {
		val serialName: String = "string"
	}
	case class Number(content: Int) extends AdvancedColors2 {
		val serialName: String = "number"
	}
	case class NumberArray(content: Vector[Int]) extends AdvancedColors2 {
		val serialName: String = "number-array"
	}
	// Comment on the last element
	case class ReallyCoolType(content: ItemDetailsFieldValue) extends AdvancedColors2 {
		val serialName: String = "really-cool-type"
	}
}

}
