/**
 * Generated by typeshare 1.1.0
 */
package com.agilebits

package onepassword {

case class ItemDetailsFieldValue (
	hello: String
)

sealed trait AdvancedColors {
	def serialName: String
}
object AdvancedColors {
	case class String(c: String) extends AdvancedColors {
		val serialName: String = "String"
	}
	case class Number(c: Int) extends AdvancedColors {
		val serialName: String = "Number"
	}
	case class NumberArray(c: Vector[Int]) extends AdvancedColors {
		val serialName: String = "NumberArray"
	}
	case class ReallyCoolType(c: ItemDetailsFieldValue) extends AdvancedColors {
		val serialName: String = "ReallyCoolType"
	}
	case class ArrayReallyCoolType(c: Vector[ItemDetailsFieldValue]) extends AdvancedColors {
		val serialName: String = "ArrayReallyCoolType"
	}
	case class DictionaryReallyCoolType(c: Map[String, ItemDetailsFieldValue]) extends AdvancedColors {
		val serialName: String = "DictionaryReallyCoolType"
	}
}

}
