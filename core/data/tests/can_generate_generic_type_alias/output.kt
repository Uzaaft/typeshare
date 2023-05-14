@file:NoLiveLiterals

package com.agilebits.onepassword

import androidx.compose.runtime.NoLiveLiterals
import kotlinx.serialization.*

typealias GenericTypeAlias<T> = List<T>

typealias NonGenericAlias = GenericTypeAlias<String?>

