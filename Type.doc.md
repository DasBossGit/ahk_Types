
Primitives:

┌ chr
└ char
┌ str
└ string
┌ number
┝ decimal
┝ int
┝ int16
┝ int32
┝ int64
┝ uint
┝ uint16
┝ uint32
┝ uint64
┝ float
┝ float32
└ float64
└ double
┌ bool
└ boolean



array<primitive>
    => Construct array with undefined length
array<primitive>[length]
    => Construct [length]-long array

object<(key:primitive)>
object<(key:primitive,key:Object<(key:primitive)>)>


int[min-max]




new Type(typeObject || templateName, )
Type.new(typeObject || templateName, )
    => construct / instantiate a Type
    Return:
        Type => if successful
        false => templateName or typeObject are invalid

Type.template.add(name, typeObject)
    => add a new Type template without constructing it
    Return:
        true => if successful
        false => if name already exists or typeObject is invalid

Type.template.remove(name)
    => remove a Type template
    Return:
        true => if successful
        false => if name does not exist

Type.validate(type, value)
    => validate a value against a Type
    Return:
        true => if value is valid
        false => if value is invalid