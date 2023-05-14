package com.agilebits

package onepassword {

class ItemDetailsFieldValue extends Serializable

sealed trait AdvancedColors {
	def serialName: String
}
object AdvancedColors {
	case class String(content: String) extends AdvancedColors {
		val serialName: String = "string"
	}
	case class Number(content: Int) extends AdvancedColors {
		val serialName: String = "number"
	}
	case class NumberArray(content: Vector[Int]) extends AdvancedColors {
		val serialName: String = "number-array"
	}
	case class ReallyCoolType(content: ItemDetailsFieldValue) extends AdvancedColors {
		val serialName: String = "reallyCoolType"
	}
}

}
