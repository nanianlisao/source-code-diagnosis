# source-code-diagnosis

This is a grocery store based on the `Rust` tool class, mainly used for various unique analysis of source code, supporting multi-threading.

## How to install

```bash
# pnpm
pnpm i @shined/source-code-diagnosis -D

# npm
npm i @shined/source-code-diagnosis -D

# yarn
yarn add @shined/source-code-diagnosis -D
```

## getUsageOfDangerStrings

Analyze the dangerous strings in the source code, usually used for third-party CDN detection.

```ts
import { getDangerStringsUsage } from "@shined/source-code-diagnosis";

let response = getDangerStringsUsage(
  ["bootcss.com", "bootcdn.com", "polyfill.com", "polyfill.io"],
  {
    cwd: "/Users/Pikachu/project",
    concurrency: 1,
  }
);
```

## getModuleMemberUsage

Analyze the usage rate of module members, generally used to analyze the number of times the exported members of a third-party package are used.

```ts
const response = getModuleMemberUsage(["antd"], {
  cwd: "/Users/Pikachu/project",
});
```

## checkSupportBrowser

共 1393 条规则，其中 189 条 chrome 要从 40 开始兼容。

operators 89 个
classes 30 个
functions 20 个
grammar 20 个
regular_expressions 19 个
statements 45 个
builtins 1182

第一批计划做 100 个。

- [ ] operators
  - [ ] addition 1
  - [ ] addition_assignment 1
  - [ ] assignment 1
  - [x] **async_function** 55
  - [x] **async_generator_function** 63
  - [x] **await** 55
    - [x] **top_level** 89
  - [ ] bitwise_and_assignment 1
  - [ ] bitwise_and 1
  - [ ] bitwise_not 1
  - [ ] bitwise_or_assignment 1
  - [ ] bitwise_or 1
  - [ ] bitwise_xor_assignment 1
  - [ ] bitwise_xor 1
  - [ ] **class** 42
  - [ ] comma 1
  - [ ] conditional 1
  - [ ] decrement 2
  - [ ] delete 1
  - [x] **destructuring** 49 🚨
    - [x] **computed_property_names** 49 🚨
    - [x] **rest_in_arrays** 49 🚨
    - [x] **rest_in_objects** 60 🚨
  - [ ] division_assignment 1
  - [ ] division 1
  - [ ] equality 1
  - [x] **exponentiation_assignment** 52
  - [x] **exponentiation** 52
  - [ ] function 1
    - [ ] **trailing_comma** 58
  - [ ] **generator_function** 49
    - [ ] **trailing_comma** 58
  - [ ] greater_than_or_equal 1
  - [ ] greater_than 1
  - [ ] grouping 1
  - [ ] **import_meta** 64
    - [ ] **resolve** 105
  - [x] **import** 63
    - [ ] **worker_support** 80
    - [x] **options_parameter** 91
  - [ ] in 1
  - [ ] increment 2
  - [ ] inequality 1
  - [ ] instanceof 1
  - [ ] left_shift_assignment 1
  - [ ] left_shift 1
  - [ ] less_than_or_equal 1
  - [ ] less_than 1
  - [ ] **logical_and_assignment** 85
  - [ ] logical_and 1
  - [ ] logical_not 1
  - [ ] **logical_or_assignment** 85
  - [ ] logical_or 1
  - [ ] multiplication_assignment 1
  - [ ] multiplication 1
  - [ ] **new_target** 46
  - [ ] new 1
  - [ ] null 1
  - [ ] **nullish_coalescing_assignment** 85
  - [ ] **nullish_coalescing** 80
  - [ ] object_initializer 1
    - [ ] **computed_property_names** 47
    - [ ] **shorthand_method_names** 47
    - [ ] **shorthand_property_names** 47
    - [ ] **spread_properties** 60
  - [ ] **optional_chaining** 80
  - [ ] property_accessors 1
  - [ ] remainder_assignment 1
  - [ ] remainder 1
  - [ ] right_shift_assignment 1
  - [ ] right_shift 1
  - [ ] **spread** 46
    - [ ] **spread_in_arrays** 46
    - [ ] **spread_in_function_calls** 46
    - [ ] **spread_in_object_literals** 60
  - [ ] strict_equality 1
  - [ ] strict_inequality 1
  - [ ] subtraction_assignment 1
  - [ ] subtraction 1
  - [ ] **super** 42
  - [ ] this 1
  - [ ] typeof 1
  - [ ] unary_negation 1
  - [ ] unary_plus 1
  - [ ] unsigned_right_shift_assignment 1
  - [ ] unsigned_right_shift 1
  - [ ] void 1
  - [ ] yield_star 39
  - [ ] yield 39
- [ ] **classes** 49
  - [ ] **constructor** 49
  - [ ] **extends** 49
  - [ ] **private_class_fields** 74
  - [ ] **private_class_fields_in** 91
  - [ ] **private_class_methods** 84
  - [ ] **public_class_fields** 72
  - [ ] **static** 49
  - [ ] **static_class_fields** 72
  - [ ] **static_initialization_blocks** 94
- [ ] functions 1
  - [ ] arguments 1
    - [ ] callee 1
    - [ ] length 1
    - [ ] **@@iterator** 52
  - [ ] **arrow_functions** 45
    - [ ] **trailing_comma** 58
  - [ ] **block_level_functions** 49
  - [ ] **default_parameters** 49
    - [ ] **destructured_parameter_with_default_value_assignment** 49
    - [ ] **parameters_without_defaults_after_default_parameters** 49
  - [ ] get 1
    - [ ] **computed_property_names** 46
  - [ ] method_definitions 39
    - [ ] **async_generator_methods** 63
    - [ ] **async_methods** 55
    - [ ] **generator_methods_not_constructable** 42
  - [ ] **rest_parameters** 47
    - [ ] **destructuring** 49
  - [ ] set 1
    - [ ] **computed_property_names** 46
- [ ] grammar 1
  - [ ] array_literals 1
  - [ ] **binary_numeric_literals** 41
  - [ ] boolean_literals 1
  - [ ] decimal_numeric_literals 1
  - [ ] **hashbang_comments** 74
  - [ ] hexadecimal_escape_sequences 1
  - [ ] hexadecimal_numeric_literals 1
  - [ ] null_literal 1
  - [ ] **numeric_separators** 75
  - [ ] **octal_numeric_literals** 41
  - [ ] regular_expression_literals 1
  - [ ] string_literals 1
  - [ ] unicode_escape_sequences 1
  - [ ] **unicode_point_escapes** 44
  - [ ] **shorthand_object_literals** 43
  - [ ] **template_literals** 41
    - [ ] **template_literal_revision** 62
  - [ ] trailing_commas 1
    - [ ] **trailing_commas_in_dynamic_import** 91
    - [ ] **trailing_commas_in_functions** 58
    - [ ] trailing_commas_in_object_literals 1
- [ ] regular_expressions 1
  - [ ] backreference 1
  - [ ] capturing_group 1
  - [ ] character_class 1
  - [ ] character_class_escape 1
  - [ ] character_escape 1
    - [ ] **unicode** 50
  - [ ] disjunction 1
  - [ ] input_boundary_assertion 1
  - [ ] literal_character 1
  - [ ] lookahead_assertion 1
  - [ ] **lookbehind_assertion** 62
  - [ ] **modifier** 125
  - [ ] **named_backreference** 64
  - [ ] **named_capturing_group** 64
    - [ ] **duplicate_named_capturing_groups** 125
  - [ ] non_capturing_group 1
  - [ ] quantifier 1
  - [ ] **unicode_character_class_escape** 64
  - [ ] wildcard 1
  - [ ] word_boundary_assertion 1
- [ ] statements 1
  - [ ] **async_function** 55
  - [ ] **async_generator_function** 63
  - [ ] block 1
  - [ ] break 1
  - [ ] **class** 49
  - [ ] const 21
  - [ ] continue 1
  - [ ] debugger 5
  - [ ] do_while 1
  - [ ] empty 3
  - [ ] **export** 61
    - [ ] **default** 61
    - [ ] **namespace** 72
  - [ ] for 1
  - [ ] **for_await_of** 63
  - [ ] for_in 1
  - [ ] for_of 38
    - [ ] **async_iterators** 63
    - [ ] **closing_iterators** 51
  - [ ] function 1
    - [ ] **trailing_comma_in_parameters** 58
  - [ ] generator_function 39
    - [ ] **IteratorResult_object** 49
    - [ ] **not_constructable_with_new** 50
    - [ ] **trailing_comma_in_parameters** 58
  - [ ] if_else 1
  - [ ] **import** 61
    - [ ] **worker_support** 80
    - [ ] **import_assertions** 91
      - [ ] **type_css** 93
      - [ ] **type_json** 91
    - [ ] **import_attributes** 123
      - [ ] **type_css** 123
      - [ ] **type_json** 123
    - [ ] **service_worker_support** 91
    - [ ] **worklet_support** 114
  - [ ] label 1
  - [ ] **let** 49
  - [ ] return 1
  - [ ] switch 1
  - [ ] throw 1
  - [ ] try_catch 1
    - [ ] **optional_catch_binding** 66
  - [ ] var 1
  - [ ] while 1
  - [ ] with 1
- [ ] builtins 24
  - [ ] Intl
    - [ ] Collator 24
      - [ ] Collator 24
        - [ ] options_caseFirst_parameter 24
        - [ ] options_collation_parameter 86
        - [ ] options_ignorePunctuation_parameter 24
        - [ ] options_localeMatcher_parameter 24
        - [ ] options_numeric_parameter 24
        - [ ] options_sensitivity_parameter 24
        - [ ] options_usage_parameter 24
      - [ ] compare 24
      - [ ] resolvedOptions 24
      - [ ] supportedLocalesOf 24
    - [ ] DateTimeFormat 24
      - [ ] DateTimeFormat 24
        - [ ] IntlLegacyConstructedSymbol 91
        - [ ] locales_parameter 24
        - [ ] options_parameter 24
          - [ ] options_calendar_parameter 80
          - [ ] options_dateStyle_parameter 76
          - [ ] options_dayPeriod_parameter 92
          - [ ] options_fractionalSecondDigits_parameter 84
          - [ ] options_hourCycle_parameter 73
          - [ ] options_numberingSystem_parameter 80
          - [ ] options_timeStyle_parameter 76
          - [ ] options_timeZone_parameter 24
            - [ ] iana_time_zones 24
          - [ ] options_timeZoneName_parameter 24
            - [ ] extended_values 95
      - [ ] format 24
      - [ ] formatRange 76
      - [ ] formatRangeToParts 76
      - [ ] formatToParts 57
      - [ ] resolvedOptions 24
        - [ ] computed_timezone 35
      - [ ] supportedLocalesOf 24
    - [ ] DisplayNames 81
      - [ ] DisplayNames 81
      - [ ] of 81
      - [ ] resolvedOptions 81
      - [ ] supportedLocalesOf 81
    - [ ] DurationFormat 16.4
      - [ ] DurationFormat 16.4
      - [ ] format 16.4
      - [ ] formatToParts 16.4
      - [ ] resolvedOptions 16.4
      - [ ] supportedLocalesOf 16.4
    - [ ] ListFormat 72
      - [ ] ListFormat 72
      - [ ] format 72
      - [ ] formatToParts 72
      - [ ] resolvedOptions 72
      - [ ] supportedLocalesOf 72
    - [ ] Locale 74
      - [ ] Locale 74
      - [ ] baseName 74
      - [ ] calendar 74
      - [ ] caseFirst 74
      - [ ] collation 74
      - [ ] getCalendars 99
      - [ ] getCollations 99
      - [ ] getHourCycles 99
      - [ ] getNumberingSystems 99
      - [ ] getTextInfo 99
      - [ ] getTimeZones 99
      - [ ] getWeekInfo 99
      - [ ] hourCycle 74
      - [ ] language 74
      - [ ] maximize 74
      - [ ] minimize 74
      - [ ] numberingSystem 74
      - [ ] numeric 74
      - [ ] region 74
      - [ ] script 74
      - [ ] toString 74
    - [ ] NumberFormat 24
      - [ ] NumberFormat 24
      - [ ] IntlLegacyConstructedSymbol 91
      - [ ] locales_parameter 24
      - [ ] options_parameter 24
        - [ ] options_compactDisplay_parameter 77
        - [ ] options_currency_parameter 24
        - [ ] options_currencyDisplay_parameter 77
        - [ ] options_currencySign_parameter 77
        - [ ] options_localeMatcher_parameter 24
        - [ ] options_maximumFractionDigits_parameter 24
        - [ ] options_maximumSignificantDigits_parameter 24
        - [ ] options_minimumFractionDigits_parameter 24
        - [ ] options_minimumIntegerDigits_parameter 24
        - [ ] options_minimumSignificantDigits_parameter 24
        - [ ] options_notation_parameter 77
        - [ ] options_numberingSystem_parameter 24
        - [ ] options_roundingIncrement_parameter 106
        - [ ] options_roundingMode_parameter 106
        - [ ] options_roundingPriority_parameter 106
        - [ ] options_signDisplay_parameter 77
          - [ ] negative 106
        - [ ] options_style_parameter 24
        - [ ] options_trailingZeroDisplay_parameter 106
        - [ ] options_unit_parameter 77
        - [ ] options_unitDisplay_parameter 77
        - [ ] options_useGrouping_parameter 24
          - [ ] string_values 106
      - [ ] format 24
        - [ ] number_parameter-string_decimal 106
      - [ ] formatRange 106
      - [ ] formatRangeToParts 106
      - [ ] formatToParts 64
      - [ ] resolvedOptions 24
      - [ ] supportedLocalesOf 24
    - [ ] PluralRules 63
      - [ ] PluralRules 63
        - [ ] options_parameter 106
          - [ ] options_roundingIncrement_parameter 116
          - [ ] options_roundingMode_parameter 117
          - [ ] options_roundingPriority_parameter 106
          - [ ] options_trailingZeroDisplay_parameter 116
      - [ ] resolvedOptions 63
      - [ ] select 63
      - [ ] selectRange 106
      - [ ] supportedLocalesOf 63
    - [ ] RelativeTimeFormat 71
      - [ ] RelativeTimeFormat 71
        - [ ] locales_parameter 71
        - [ ] options_parameter
          - [ ] options_localeMatcher_parameter 71
          - [ ] options_numberingSystem_parameter 71
          - [ ] options_numeric_parameter 71
      - [ ] format 71
      - [ ] formatToParts 71
      - [ ] resolvedOptions 71
        - [ ] numberingSystem 73
      - [ ] supportedLocalesOf 71
    - [ ] Segmenter 87
      - [ ] Segmenter 87
      - [ ] resolvedOptions 87
      - [ ] segment 87
      - [ ] supportedLocalesOf 87
    - [ ] Segments 87
      - [ ] containing 87
      - [ ] @@iterator 87
  - [ ] Temporal
    - [ ] Calendar
      - [ ] Calendar
      - [ ] dateAdd
      - [ ] dateFromFields
      - [ ] dateUntil
      - [ ] day
      - [ ] dayOfWeek
      - [ ] dayOfYear
      - [ ] daysInMonth
      - [ ] daysInWeek
      - [ ] daysInYear
      - [ ] era
      - [ ] eraYear
      - [ ] fields
      - [ ] from
      - [ ] id
      - [ ] inLeapYear
      - [ ] mergeFields
      - [ ] month
      - [ ] monthCode
      - [ ] monthDayFromFields
      - [ ] monthsInYear
      - [ ] toJSON
      - [ ] toString
      - [ ] weekOfYear
      - [ ] year
      - [ ] yearMonthFromFields
    - [ ] Duration
      - [ ] Duration
      - [ ] abs
      - [ ] add
      - [ ] blank
      - [ ] compare
      - [ ] days
      - [ ] from
      - [ ] hours
      - [ ] microseconds
      - [ ] milliseconds
      - [ ] minutes
      - [ ] months
      - [ ] nanoseconds
      - [ ] negated
      - [ ] round
      - [ ] seconds
      - [ ] sign
      - [ ] subtract
      - [ ] toJSON
      - [ ] toLocaleString
      - [ ] toString
      - [ ] total
      - [ ] valueOf
      - [ ] weeks
      - [ ] with
      - [ ] years
    - [ ] Instant
      - [ ] Instant
      - [ ] add
      - [ ] compare
      - [ ] epochMicroseconds
      - [ ] epochMilliseconds
      - [ ] epochNanoseconds
      - [ ] epochSeconds
      - [ ] equals
      - [ ] from
      - [ ] fromEpochMicroseconds
      - [ ] fromEpochMilliseconds
      - [ ] fromEpochNanoseconds
      - [ ] fromEpochSeconds
      - [ ] round
      - [ ] since
      - [ ] subtract
      - [ ] toJSON
      - [ ] toLocaleString
      - [ ] toString
      - [ ] toZonedDateTime
      - [ ] toZonedDateTimeISO
      - [ ] until
      - [ ] valueOf
    - [ ] Now
      - [ ] instant
      - [ ] plainDate
      - [ ] plainDateISO
      - [ ] plainDateTime
      - [ ] plainDateTimeISO
      - [ ] timeZone
      - [ ] zonedDateTime
      - [ ] zonedDateTimeISO
    - [ ] PlainDate
      - [ ] PlainDate
      - [ ] add
      - [ ] calendar
      - [ ] compare
      - [ ] day
      - [ ] dayOfWeek
      - [ ] dayOfYear
      - [ ] daysInMonth
      - [ ] daysInWeek
      - [ ] daysInYear
      - [ ] equals
      - [ ] era
      - [ ] eraYear
      - [ ] from
      - [ ] getISOFields
      - [ ] inLeapYear
      - [ ] month
      - [ ] monthCode
      - [ ] monthsInYear
      - [ ] since
      - [ ] subtract
      - [ ] toJSON
      - [ ] toLocaleString
      - [ ] toPlainDateTime
      - [ ] toPlainMonthDay
      - [ ] toPlainYearMonth
      - [ ] toString
      - [ ] toZonedDateTime
      - [ ] until
      - [ ] valueOf
      - [ ] weekOfYear
      - [ ] with
      - [ ] withCalendar
      - [ ] year
    - [ ] PlainDateTime
      - [ ] PlainDateTime
      - [ ] add
      - [ ] calendar
      - [ ] compare
      - [ ] day
      - [ ] dayOfWeek
      - [ ] dayOfYear
      - [ ] daysInMonth
      - [ ] daysInWeek
      - [ ] daysInYear
      - [ ] equals
      - [ ] era
      - [ ] eraYear
      - [ ] from
      - [ ] getISOFields
      - [ ] hour
      - [ ] inLeapYear
      - [ ] microsecond
      - [ ] millisecond
      - [ ] minute
      - [ ] month
      - [ ] monthCode
      - [ ] monthsInYear
      - [ ] nanosecond
      - [ ] round
      - [ ] second
      - [ ] since
      - [ ] subtract
      - [ ] toJSON
      - [ ] toLocaleString
      - [ ] toPlainDate
      - [ ] toPlainMonthDay
      - [ ] toPlainTime
      - [ ] toPlainYearMonth
      - [ ] toString
      - [ ] toZonedDateTime
      - [ ] until
      - [ ] valueOf
      - [ ] weekOfYear
      - [ ] with
      - [ ] withCalendar
      - [ ] withPlainDate
      - [ ] withPlainTime
      - [ ] year
    - [ ] PlainMonthDay
      - [ ] PlainMonthDay
      - [ ] calendar
      - [ ] day
      - [ ] equals
      - [ ] from
      - [ ] getISOFields
      - [ ] monthCode
      - [ ] toJSON
      - [ ] toLocaleString
      - [ ] toPlainDate
      - [ ] toString
      - [ ] valueOf
      - [ ] with
    - [ ] PlainTime
      - [ ] PlainTime
      - [ ] add
      - [ ] calendar
      - [ ] compare
      - [ ] equals
      - [ ] from
      - [ ] getISOFields
      - [ ] hour
      - [ ] microsecond
      - [ ] millisecond
      - [ ] minute
      - [ ] nanosecond
      - [ ] round
      - [ ] second
      - [ ] since
      - [ ] subtract
      - [ ] toJSON
      - [ ] toLocaleString
      - [ ] toPlainDateTime
      - [ ] toString
      - [ ] toZonedDateTime
      - [ ] until
      - [ ] valueOf
      - [ ] with
    - [ ] PlainYearMonth
      - [ ] PlainYearMonth
      - [ ] add
      - [ ] calendar
      - [ ] compare
      - [ ] daysInMonth
      - [ ] daysInYear
      - [ ] equals
      - [ ] era
      - [ ] eraYear
      - [ ] from
      - [ ] getISOFields
      - [ ] inLeapYear
      - [ ] month
      - [ ] monthCode
      - [ ] monthsInYear
      - [ ] since
      - [ ] subtract
      - [ ] toJSON
      - [ ] toLocaleString
      - [ ] toPlainDate
      - [ ] toString
      - [ ] until
      - [ ] valueOf
      - [ ] with
      - [ ] year
    - [ ] TimeZone
      - [ ] TimeZone
      - [ ] from
      - [ ] getInstantFor
      - [ ] getNextTransition
      - [ ] getOffsetNanosecondsFor
      - [ ] getOffsetStringFor
      - [ ] getPlainDateTimeFor
      - [ ] getPossibleInstantsFor
      - [ ] getPreviousTransition
      - [ ] id
      - [ ] toJSON
      - [ ] toString
    - [ ] ZonedDateTime
      - [ ] ZonedDateTime
      - [ ] add
      - [ ] calendar
      - [ ] compare
      - [ ] day
      - [ ] dayOfWeek
      - [ ] dayOfYear
      - [ ] daysInMonth
      - [ ] daysInWeek
      - [ ] daysInYear
      - [ ] epochMicroseconds
      - [ ] epochMilliseconds
      - [ ] epochNanoseconds
      - [ ] epochSeconds
      - [ ] equals
      - [ ] era
      - [ ] eraYear
      - [ ] from
      - [ ] getISOFields
      - [ ] getTimeZoneTransition
      - [ ] hour
      - [ ] hoursInDay
      - [ ] inLeapYear
      - [ ] microsecond
      - [ ] millisecond
      - [ ] minute
      - [ ] month
      - [ ] monthCode
      - [ ] monthsInYear
      - [ ] nanosecond
      - [ ] offset
      - [ ] offsetNanoseconds
      - [ ] round
      - [ ] second
      - [ ] since
      - [ ] startOfDay
      - [ ] subtract
      - [ ] timeZone
      - [ ] toInstant
      - [ ] toJSON
      - [ ] toLocaleString
      - [ ] toPlainDate
      - [ ] toPlainDateTime
      - [ ] toPlainMonthDay
      - [ ] toPlainTime
      - [ ] toPlainYearMonth
      - [ ] toString
      - [ ] until
      - [ ] valueOf
      - [ ] weekOfYear
      - [ ] with
      - [ ] withCalendar
      - [ ] withPlainDate
      - [ ] withPlainTime
      - [ ] withTimeZone
      - [ ] year
  - [ ] AggregateError 85
    - [ ] AggregateError 85
    - [ ] errors 85
    - [ ] serializable_object N/A
  - [ ] Array 1
    - [ ] Array 1
    - [ ] at 92
    - [ ] concat 1
    - [ ] copyWithin 45
    - [ ] entries 38
    - [ ] every 1
    - [ ] fill 45
    - [ ] filter 1
    - [ ] find 45
    - [ ] findIndex 45
    - [ ] findLast 97
    - [ ] findLastIndex 97
    - [ ] flat 69
    - [ ] flatMap 69
    - [ ] forEach 1
    - [ ] from 45
    - [ ] fromAsync 121
    - [ ] includes 47
    - [ ] indexOf 1
    - [ ] isArray 4
    - [ ] join 1
    - [ ] keys 38
    - [ ] lastIndexOf 1
    - [ ] length 1
    - [ ] map 1
    - [ ] of 45
    - [ ] pop 1
    - [ ] push 1
    - [ ] reduce 3
    - [ ] reduceRight 3
    - [ ] reverse 1
    - [ ] shift 1
    - [ ] slice 1
    - [ ] some 1
    - [ ] sort 1
      - [ ] stable_sorting 70
    - [ ] splice 1
    - [ ] toLocaleString 1
      - [ ] locales_parameter 24
      - [ ] options_parameter 24
    - [ ] toReversed 110
    - [ ] toSorted 110
    - [ ] toSpliced 110
    - [ ] toString 1
    - [ ] unshift 1
    - [ ] values 66
    - [ ] with 110
    - [ ] @@iterator 38
    - [ ] @@species 51
    - [ ] @@unscopables 38
  - [ ] ArrayBuffer 7
    - [ ] ArrayBuffer 7
    - [ ] maxByteLength_option 111
    - [ ] byteLength 7
    - [ ] detached 114
    - [ ] isView 32
    - [ ] maxByteLength 111
    - [ ] resizable 111
    - [ ] resize 111
    - [ ] slice 17
    - [ ] transfer 114
    - [ ] transferToFixedLength 114
    - [ ] @@species 51
  - [ ] AsyncFunction 55
    - [ ] AsyncFunction 55
  - [ ] AsyncGenerator 63
    - [ ] next 63
    - [ ] return 63
    - [ ] throw 63
  - [ ] AsyncGeneratorFunction 63
    - [ ] AsyncGeneratorFunction 63
  - [ ] AsyncIterator 63
    - [ ] @@asyncIterator 63
  - [ ] Atomics 68
    - [ ] Atomic_operations_on_non_shared_buffers 79
    - [ ] add 68
    - [ ] and 68
    - [ ] compareExchange 68
    - [ ] exchange 68
    - [ ] isLockFree 68
    - [ ] load 68
    - [ ] notify 68
    - [ ] or 68
    - [ ] store 68
    - [ ] sub 68
    - [ ] wait 68
    - [ ] waitAsync 87
    - [ ] xor 68
  - [ ] BigInt 67
    - [ ] BigInt 67
    - [ ] asIntN 67
    - [ ] asUintN 67
    - [ ] toLocaleString 67
      - [ ] locales_parameter 76
      - [ ] options_parameter 76
    - [ ] toString 67
    - [ ] valueOf 67
  - [ ] BigInt64Array 67
    - [ ] BigInt64Array 67
  - [ ] BigUint64Array 67
    - [ ] BigUint64Array 67
  - [ ] Boolean 1
    - [ ] Boolean 1
    - [ ] toString 1
    - [ ] valueOf 1
  - [ ] DataView 9
    - [ ] DataView 9
      - [ ] sharedarraybuffer_support 68
    - [ ] buffer 9
    - [ ] byteLength 9
    - [ ] byteOffset 9
    - [ ] getBigInt64 67
    - [ ] getBigUint64 67
    - [ ] getFloat16 129
    - [ ] getFloat32 9
    - [ ] getFloat64 9
    - [ ] getInt16 9
    - [ ] getInt32 9
    - [ ] getInt8 9
    - [ ] getUint16 9
    - [ ] getUint32 9
    - [ ] getUint8 9
    - [ ] setBigInt64 67
    - [ ] setBigUint64 67
    - [ ] setFloat16 129
    - [ ] setFloat32 9
    - [ ] setFloat64 9
    - [ ] setInt16 9
    - [ ] setInt32 9
    - [ ] setInt8 9
    - [ ] setUint16 9
    - [ ] setUint32 9
    - [ ] setUint8 9
  - [ ] Date 1
    - [ ] Date 1
    - [ ] UTC 1
      - [ ] optional_monthIndex 57
    - [ ] getDate 1
    - [ ] getDay 1
    - [ ] getFullYear 1
    - [ ] getHours 1
    - [ ] getMilliseconds 1
    - [ ] getMinutes 1
    - [ ] getMonth 1
    - [ ] getSeconds 1
    - [ ] getTime 1
    - [ ] getTimezoneOffset 1
    - [ ] getUTCDate 1
    - [ ] getUTCDay 1
    - [ ] getUTCFullYear 1
    - [ ] getUTCHours 1
    - [ ] getUTCMilliseconds 1
    - [ ] getUTCMinutes 1
    - [ ] getUTCMonth 1
    - [ ] getUTCSeconds 1
    - [ ] getYear 1
    - [ ] now 1
    - [ ] parse 1
      - [ ] iso_8601 6
    - [ ] setDate 1
    - [ ] setFullYear 1
    - [ ] setHours 1
    - [ ] setMilliseconds 1
    - [ ] setMinutes 1
    - [ ] setMonth 1
    - [ ] setSeconds 1
    - [ ] setTime 1
    - [ ] setUTCDate 1
    - [ ] setUTCFullYear 1
    - [ ] setUTCHours 1
    - [ ] setUTCMilliseconds 1
    - [ ] setUTCMinutes 1
    - [ ] setUTCMonth 1
    - [ ] setUTCSeconds 1
    - [ ] setYear 1
    - [ ] toDateString 1
    - [ ] toGMTString 1
    - [ ] toISOString 3
    - [ ] toJSON 3
    - [ ] toLocaleDateString 1
      - [ ] iana_time_zone_names 24
      - [ ] locales_parameter 24
      - [ ] options_parameter 24
    - [ ] toLocaleString 1
      - [ ] iana_time_zone_names 24
      - [ ] locales_parameter 24
      - [ ] options_parameter 24
    - [ ] toLocaleTimeString 1
      - [ ] iana_time_zone_names 24
      - [ ] locales_parameter 24
      - [ ] options_parameter 24
    - [ ] toString 1
    - [ ] toTimeString 1
    - [ ] toUTCString 1
    - [ ] valueOf 1
    - [ ] @@toPrimitive 47
  - [ ] Error 1
    - [ ] Error 1
      - [ ] fileName_parameter 1
      - [ ] lineNumber_parameter 1
      - [ ] options_cause_parameter 93
    - [ ] cause 93
      - [ ] displayed_in_console 91
    - [ ] columnNumber 1
    - [ ] fileName 1
    - [ ] lineNumber 1
    - [ ] message 1
    - [ ] name 1
    - [ ] serializable_object 77
    - [ ] stack 3
    - [ ] toString 1
  - [ ] EvalError 1
    - [ ] EvalError 1
    - [ ] serializable_object 77
  - [ ] FinalizationRegistry 84
    - [ ] FinalizationRegistry 84
    - [ ] register 84
    - [ ] symbol_as_target 109
    - [ ] unregister 84
  - [ ] Float16Array 129
    - [ ] Float16Array 129
  - [ ] Float32Array 7
    - [ ] Float32Array 7
      - [ ] constructor_without_parameters 7
      - [ ] iterable_allowed 39
  - [ ] Float64Array 7
    - [ ] Float64Array 7
      - [ ] constructor_without_parameters 7
      - [ ] iterable_allowed 39
  - [ ] Function 1
    - [ ] Function 1
    - [ ] apply 1
      - [ ] generic_arrays_as_arguments 17
    - [ ] arguments 1
    - [ ] bind 7
    - [ ] call 1
    - [ ] caller 1
    - [ ] displayName 13
    - [ ] length 1
      - [ ] configurable_true 43
    - [ ] name 1
      - [ ] configurable_true 43
      - [ ] inferred_names 51
    - [ ] toString 1
      - [ ] toString_revision 66
    - [ ] @@hasInstance 50
  - [ ] Generator 39
    - [ ] next 39
    - [ ] return 50
    - [ ] throw 39
  - [ ] GeneratorFunction 39
    - [ ] GeneratorFunction 39
  - [ ] globals
    - [ ] Infinity 1
    - [ ] NaN 1
    - [ ] decodeURI 1
    - [ ] decodeURIComponent 1
    - [ ] encodeURI 1
    - [ ] encodeURIComponent 1
    - [ ] escape 1
    - [ ] eval 1
    - [ ] globalThis 71
    - [ ] isFinite 1
    - [ ] isNaN 1
    - [ ] parseFloat 1
    - [ ] parseInt 1
      - [ ] leading_zero_strings_as_decimal 23
    - [ ] undefined 1
    - [ ] unescape 1
  - [ ] Int8Array 7
    - [ ] Int8Array 7
      - [ ] constructor_without_parameters 7
      - [ ] iterable_allowed 39
  - [ ] Int16Array 7
    - [ ] Int16Array 7
      - [ ] constructor_without_parameters 7
      - [ ] iterable_allowed 39
  - [ ] Int32Array 7
    - [ ] Int32Array 7
      - [ ] constructor_without_parameters 7
      - [ ] iterable_allowed 39
  - [ ] InternalError 1
    - [ ] InternalError 1
  - [ ] Intl 24
    - [ ] getCanonicalLocales 54
    - [ ] supportedValuesOf 99
  - [ ] Iterator 38
    - [ ] Iterator 122
    - [ ] drop 122
    - [ ] every 122
    - [ ] filter 122
    - [ ] find 122
    - [ ] flatMap 122
    - [ ] forEach 122
    - [ ] from 122
    - [ ] map 122
    - [ ] reduce 122
    - [ ] some 122
    - [ ] take 122
    - [ ] toArray 122
    - [ ] @@iterator 38
  - [ ] JSON 3
    - [ ] isRawJSON 114
    - [ ] json_superset 66
    - [ ] parse 3
      - [ ] reviver_parameter_context_argument 114
    - [ ] rawJSON 114
    - [ ] stringify 3
      - [ ] well_formed_stringify 72
  - [ ] Map 38
    - [ ] Map 38
      - [ ] iterable_allowed 38
      - [ ] null_allowed 38
    - [ ] clear 38
    - [ ] delete 38
    - [ ] entries 38
    - [ ] forEach 38
    - [ ] get 38
    - [ ] groupBy 117
    - [ ] has 38
    - [ ] key_equality_for_zeros 38
    - [ ] keys 38
    - [ ] set 38
    - [ ] size 38
    - [ ] values 38
    - [ ] @@iterator 38
    - [ ] @@species 51
  - [ ] Math 1
    - [ ] E 1
    - [ ] LN10 1
    - [ ] LN2 1
    - [ ] LOG10E 1
    - [ ] LOG2E 1
    - [ ] PI 1
    - [ ] SQRT1_2 1
    - [ ] SQRT2 1
    - [ ] abs 1
    - [ ] acos 1
    - [ ] acosh 38
    - [ ] asin 1
    - [ ] asinh 38
    - [ ] atan 1
    - [ ] atan2 1
    - [ ] atanh 38
    - [ ] cbrt 38
    - [ ] ceil 1
    - [ ] clz32 38
    - [ ] cos 1
    - [ ] cosh 38
    - [ ] exp 1
    - [ ] expm1 38
    - [ ] f16round 129
    - [ ] floor 1
    - [ ] fround 38
    - [ ] hypot 38
    - [ ] imul 28
    - [ ] log 1
    - [ ] log10 38
    - [ ] log1p 38
    - [ ] log2 38
    - [ ] max 1
    - [ ] min 1
    - [ ] pow 1
    - [ ] random 1
    - [ ] round 1
    - [ ] sign 38
    - [ ] sin 1
    - [ ] sinh 38
    - [ ] sqrt 1
    - [ ] tan 1
    - [ ] tanh 38
    - [ ] trunc 38
  - [ ] Number 1
    - [ ] EPSILON 34
    - [ ] MAX_SAFE_INTEGER 34
    - [ ] MAX_VALUE 1
    - [ ] MIN_SAFE_INTEGER 34
    - [ ] MIN_VALUE 1
    - [ ] NaN 1
    - [ ] NEGATIVE_INFINITY 1
    - [ ] Number 1
    - [ ] POSITIVE_INFINITY 1
    - [ ] isFinite 19
    - [ ] isInteger 34
    - [ ] isNaN 25
    - [ ] isSafeInteger 34
    - [ ] parseFloat 34
    - [ ] parseInt 34
    - [ ] toExponential 1
    - [ ] toFixed 1
    - [ ] toLocaleString 1
      - [ ] locales_parameter 24
      - [ ] options_parameter 24
    - [ ] toPrecision 1
    - [ ] toString 1
    - [ ] valueOf 1
  - [ ] Object 1
    - [ ] Object 1
    - [ ] assign 45
    - [ ] constructor 1
    - [ ] create 5
    - [ ] **defineGetter** 1
    - [ ] defineProperties 5
    - [ ] defineProperty 5
    - [ ] **defineSetter** 1
    - [ ] entries 54
    - [ ] freeze 6
    - [ ] fromEntries 73
    - [ ] getOwnPropertyDescriptor 5
    - [ ] getOwnPropertyDescriptors 54
    - [ ] getOwnPropertyNames 5
    - [ ] getOwnPropertySymbols 38
    - [ ] getPrototypeOf 5
    - [ ] groupBy 117
    - [ ] hasOwn 93
    - [ ] hasOwnProperty 1
    - [ ] is 19
    - [ ] isExtensible 6
    - [ ] isFrozen 6
    - [ ] isPrototypeOf 1
    - [ ] isSealed 6
    - [ ] keys 5
    - [ ] **lookupGetter** 1
    - [ ] **lookupSetter** 1
    - [ ] preventExtensions 6
      - [ ] ES2015_behavior 44
    - [ ] propertyIsEnumerable 1
    - [ ] **proto** 1
    - [ ] seal 6
    - [ ] setPrototypeOf 34
    - [ ] toLocaleString 1
    - [ ] toString 1
    - [ ] valueOf 1
    - [ ] values 54
  - [ ] Promise 32
    - [ ] Promise 32
    - [ ] all 32
    - [ ] allSettled 76
    - [ ] any 85
    - [ ] catch 32
    - [ ] finally 63
    - [ ] incumbent_settings_object_tracking 50
    - [ ] race 32
    - [ ] reject 32
    - [ ] resolve 32
    - [ ] then 32
    - [ ] try 128
    - [ ] withResolvers 119
    - [ ] @@species 51
  - [ ] Proxy 49
    - [ ] Proxy 49
    - [ ] handler
      - [ ] apply 49
      - [ ] construct 49
      - [ ] defineProperty 49
      - [ ] deleteProperty 49
      - [ ] get 49
      - [ ] getOwnPropertyDescriptor 49
      - [ ] getPrototypeOf 49
      - [ ] has 49
      - [ ] isExtensible 49
      - [ ] ownKeys 49
      - [ ] preventExtensions 49
      - [ ] set 49
      - [ ] setPrototypeOf 49
    - [ ] revocable 63
  - [ ] RangeError 1
    - [ ] RangeError 1
    - [ ] serializable_object 77
  - [ ] ReferenceError 1
    - [ ] ReferenceError 1
    - [ ] serializable_object 77
  - [ ] Reflect 49
    - [ ] apply 49
    - [ ] construct 49
    - [ ] defineProperty 49
    - [ ] deleteProperty 49
    - [ ] get 49
    - [ ] getOwnPropertyDescriptor 49
    - [ ] getPrototypeOf 49
    - [ ] has 49
    - [ ] isExtensible 49
    - [ ] ownKeys 49
    - [ ] preventExtensions 49
    - [ ] set 49
    - [ ] setPrototypeOf 49
  - [ ] RegExp 1
    - [ ] RegExp 1
    - [ ] compile 1
    - [ ] dotAll 62
    - [ ] exec 1
    - [ ] flags 49
    - [ ] global 1
      - [ ] prototype_accessor 48
    - [ ] hasIndices 90
    - [ ] ignoreCase 1
      - [ ] prototype_accessor 48
    - [ ] input 1
    - [ ] lastIndex 1
    - [ ] lastMatch 1
    - [ ] lastParen 1
    - [ ] leftContext 1
    - [ ] multiline 1
      - [ ] prototype_accessor 48
    - [ ] n 1
    - [ ] rightContext 1
    - [ ] source 1
      - [ ] empty_regex_string 6
      - [ ] Escaping 73
      - [ ] prototype_accessor 48
    - [ ] sticky 49
      - [ ] Anchored sticky flag behavior per ES2015 49
      - [ ] prototype_accessor 49
    - [ ] test 1
    - [ ] toString 1
      - [ ] Escaping 73
    - [ ] unicode 50
    - [ ] unicodeSets 112
    - [ ] @@match 50
    - [ ] @@matchAll 73
    - [ ] @@replace 50
    - [ ] @@search 50
    - [ ] @@species 51
    - [ ] @@split 50
  - [ ] Set 38
    - [ ] Set 38
    - [ ] iterable_allowed 38
    - [ ] null_allowed 38
    - [ ] add 38
    - [ ] clear 38
    - [ ] delete 38
    - [ ] difference 122
    - [ ] entries 38
    - [ ] forEach 38
    - [ ] has 38
    - [ ] intersection 122
    - [ ] isDisjointFrom 122
    - [ ] isSubsetOf 122
    - [ ] isSupersetOf 122
    - [ ] key_equality_for_zeros 38
    - [ ] keys 38
    - [ ] size 38
    - [ ] symmetricDifference 122
    - [ ] union 122
    - [ ] values 38
    - [ ] @@iterator 43
    - [ ] @@species 51
  - [ ] SharedArrayBuffer 68
    - [ ] SharedArrayBuffer 68
    - [ ] maxByteLength_option 111
    - [ ] byteLength 68
    - [ ] grow 111
    - [ ] growable 111
    - [ ] maxByteLength 111
    - [ ] slice 68
    - [ ] @@species 68
  - [ ] String 1
    - [ ] String 1
    - [ ] anchor 1
    - [ ] at 92
    - [ ] big 1
    - [ ] blink 1
    - [ ] bold 1
    - [ ] charAt 1
    - [ ] charCodeAt 1
    - [ ] codePointAt 41
    - [ ] concat 1
    - [ ] endsWith 41
    - [ ] fixed 1
    - [ ] fontcolor 1
    - [ ] fontsize 1
    - [ ] fromCharCode 1
    - [ ] fromCodePoint 41
    - [ ] includes 41
    - [ ] indexOf 1
    - [ ] isWellFormed 111
    - [ ] italics 1
    - [ ] lastIndexOf 1
    - [ ] length 1
    - [ ] link 1
    - [ ] localeCompare 1
      - [ ] locales_parameter 24
      - [ ] options_parameter 24
    - [ ] match 1
    - [ ] matchAll 73
    - [ ] normalize 34
    - [ ] padEnd 57
    - [ ] padStart 57
    - [ ] raw 41
    - [ ] repeat 41
    - [ ] replace 1
    - [ ] replaceAll 85
    - [ ] search 1
    - [ ] slice 1
    - [ ] small 1
    - [ ] split 1
    - [ ] startsWith 41
    - [ ] strike 1
    - [ ] sub 1
    - [ ] substr 1
    - [ ] substring 1
    - [ ] sup 1
    - [ ] toLocaleLowerCase 1
      - [ ] locale 58
    - [ ] toLocaleUpperCase 1
      - [ ] locale 58
    - [ ] toLowerCase 1
    - [ ] toString 1
    - [ ] toUpperCase 1
    - [ ] toWellFormed 111
    - [ ] trim 4
    - [ ] trimEnd 66
    - [ ] trimStart 66
    - [ ] unicode_code_point_escapes 1
    - [ ] valueOf 1
    - [ ] @@iterator 38
  - [ ] Symbol 38
    - [ ] Symbol 38
    - [ ] asyncIterator 63
    - [ ] description 70
    - [ ] for 40
    - [ ] hasInstance 50
    - [ ] isConcatSpreadable 48
    - [ ] iterator 43
    - [ ] keyFor 40
    - [ ] match 50
    - [ ] matchAll 73
    - [ ] replace 50
    - [ ] search 50
    - [ ] species 51
    - [ ] split 50
    - [ ] toPrimitive 47
    - [ ] toString 38
    - [ ] toStringTag 49
      - [ ] dom_objects 50
    - [ ] unscopables 38
    - [ ] valueOf 38
    - [ ] @@toPrimitive 47
  - [ ] SyntaxError 1
    - [ ] SyntaxError 1
    - [ ] serializable_object 77
  - [ ] Temporal
    - [ ] Temporal API
  - [ ] TypedArray 7
    - [ ] BYTES_PER_ELEMENT 7
    - [ ] at 92
    - [ ] buffer 7
    - [ ] byteLength 7
    - [ ] byteOffset 7
    - [ ] constructor_without_parameters 7
    - [ ] copyWithin 45
    - [ ] entries 45
    - [ ] every 45
    - [ ] fill 45
    - [ ] filter 45
    - [ ] find 45
    - [ ] findIndex 45
    - [ ] findLast 97
    - [ ] findLastIndex 97
    - [ ] forEach 45
    - [ ] from 45
    - [ ] includes 47
    - [ ] index_properties_not_consulting_prototype 7
    - [ ] indexOf 45
    - [ ] iterable_in_constructor 39
    - [ ] join 45
    - [ ] keys 38
    - [ ] lastIndexOf 45
    - [ ] length 7
    - [ ] map 45
    - [ ] name 7
    - [ ] named_properties 7
    - [ ] of 45
    - [ ] reduce 45
    - [ ] reduceRight 45
    - [ ] reverse 45
    - [ ] set 7
    - [ ] slice 45
    - [ ] some 45
    - [ ] sort 45
    - [ ] subarray 7
    - [ ] toLocaleString 7
    - [ ] toReversed 110
    - [ ] toSorted 110
    - [ ] toString 7
    - [ ] values 38
    - [ ] with 110
    - [ ] @@iterator 38
    - [ ] @@species 51
  - [ ] TypeError 1
    - [ ] TypeError 1
    - [ ] serializable_object 77
  - [ ] Uint8Array 7
    - [ ] Uint8Array 7
      - [ ] constructor_without_parameters 7
      - [ ] iterable_allowed 39
  - [ ] Uint8ClampedArray 7
    - [ ] Uint8ClampedArray 7
      - [ ] constructor_without_parameters 7
      - [ ] iterable_allowed 39
  - [ ] Uint16Array 7
    - [ ] Uint16Array 7
      - [ ] constructor_without_parameters 7
      - [ ] iterable_allowed 39
  - [ ] Uint32Array 7
    - [ ] Uint32Array 7
      - [ ] constructor_without_parameters 7
      - [ ] iterable_allowed 39
  - [ ] URIError 1
    - [ ] URIError 1
    - [ ] serializable_object 77
  - [ ] WeakMap 36
    - [ ] WeakMap 36
      - [ ] iterable_allowed 38
      - [ ] null_allowed 36
    - [ ] delete 36
    - [ ] get 36
    - [ ] has 36
    - [ ] set 36
    - [ ] symbol_as_keys 109
  - [ ] WeakRef 84
    - [ ] WeakRef 84
    - [ ] deref 84
    - [ ] symbol_as_target 109
  - [ ] WeakSet 36
    - [ ] WeakSet 36
      - [ ] iterable_allowed 38
      - [ ] null_allowed 36
    - [ ] add 36
    - [ ] delete 36
    - [ ] has 36
    - [ ] symbol_as_keys 109
