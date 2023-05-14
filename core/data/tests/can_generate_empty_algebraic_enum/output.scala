package com.agilebits

package onepassword {

class AddressDetails extends Serializable

sealed trait Address {
	def serialName: String
}
object Address {
	case class FixedAddress(content: AddressDetails) extends Address {
		val serialName: String = "FixedAddress"
	}
	case object NoFixedAddress extends Address {
		val serialName: String = "NoFixedAddress"
	}
}

}
