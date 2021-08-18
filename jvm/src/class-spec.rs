//! As per [JVM specification](https://docs.oracle.com/javase/specs/jvms/se16/html/jvms-4.html), the class specification is a sequence of
//! UTF-8 strings.
//! Defined as below
//```
// ClassFile {
//     u4             magic;
//     u2             minor_version;
//     u2             major_version;
//     u2             constant_pool_count;
//     cp_info        constant_pool[constant_pool_count-1];
//     u2             access_flags;
//     u2             this_class;
//     u2             super_class;
//     u2             interfaces_count;
//     u2             interfaces[interfaces_count];
//     u2             fields_count;
//     field_info     fields[fields_count];
//     u2             methods_count;
//     method_info    methods[methods_count];
//     u2             attributes_count;
//     attribute_info attributes[attributes_count];
// }
//```
//

struct ClassFile {
    magic: u32,
    minor_version: u16,
    major_version: u16,
    constant_pool_count: u16,
    constant_pool: Vec<ConstantPoolInfo>,
    access_flags: u16,
    this_class: u16,
    super_class: u16,
    interfaces_count: u16,
    interfaces: Vec<u16>,
    fields_count: u16,
    fields: Vec<FieldInfo>,
    methods_count: u16,
    methods: Vec<MethodInfo>,
    attributes_count: u16,
    attributes: Vec<AttributeInfo>,
}

// Constant pool info
//
// ```
// cp_info {
//     u1 tag;
//     u1 info[];
// }
// ```

struct ConstantPoolInfo {
    tag: u8,
    info: Vec<u8>,
}

// Field info
//
// ```
// field_info {
//     u2             access_flags;
//     u2             name_index;
//     u2             descriptor_index;
//     u2             attributes_count;
//     attribute_info attributes[attributes_count];
// }
// ```

struct FieldInfo {
    access_flags: u16,
    name_index: u16,
    descriptor_index: u16,
    attributes_count: u16,
    attributes: Vec<AttributeInfo>,
}

// Method info
//
// ```
// method_info {
//     u2             access_flags;
//     u2             name_index;
//     u2             descriptor_index;
//     u2             attributes_count;
//     attribute_info attributes[attributes_count];
// }
// ```

struct MethodInfo {
    access_flags: u16,
    name_index: u16,
    descriptor_index: u16,
    attributes_count: u16,
    attributes: Vec<AttributeInfo>,
}

// Attribute info
//
// ```
// attribute_info {
//     u2 attribute_name_index;
//     u4 attribute_length;
//     u1 info[attribute_length];
// }
// ```

struct AttributeInfo {
    attribute_name_index: u16,
    attribute_length: u32,
    info: Vec<u8>,
}
