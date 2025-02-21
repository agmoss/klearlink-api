# KlearLink API

## Table of Contents
1. [Authentication](#authentication)
2. [Error Handling](#error-handling)
3. [Endpoints](#endpoints)
   - [Submit a consumer credit record](#1-submit-a-consumer-credit-record)
   - [Update a consumer credit record](#2-update-a-consumer-credit-record)
   - [View a submitted consumer credit record](#3-view-a-submitted-consumer-credit-record)
   - [View Consumer Match](#4-view-consumer-match)
4. [Appendix](#appendix)
   - [Definitions](#a-definitions)
   - [Data Standards](#b-data-standards)

## Authentication

All API requests must include authentication credentials in the request headers. We use API key-based authentication.

**Headers**:

| Header       | Value  | Description                  |
| ------------ | ------ | ---------------------------- |
| `X-API-Key`  | string | Your unique API key (UUIDV4) |
| `X-Username` | string | Your registered username     |

**Example Request Headers**:

```http
X-API-Key: your_api_key_here
X-Username: your_username
```

:::warning
Keep your API key secure and never share it. If you believe your API key has been compromised, contact support immediately for a replacement.
:::

## Error Handling

The API uses standard HTTP status codes to indicate the success or failure of an API request. Common error codes include:

- **400 Bad Request**: The request was invalid or cannot be otherwise served.
- **401 Unauthorized**: Authentication credentials were missing or incorrect.
- **404 Not Found**: The requested resource could not be found.
- **409 Conflict**: The request could not be completed due to a conflict with the current state of the resource.


## 1. Submit a consumer credit record

**Endpoint**: `/consumer-credit/{id}`

**Method**: `PUT`

**Description**: Create a new consumer credit record in the system.

**Path Parameters**:

| Parameter | Type   | Description                                      |
| --------- | ------ | ------------------------------------------------ |
| id        | string | Unique identifier for the consumer credit record |

**Response Codes**:

| Code | Description                                                          |
| ---- | -------------------------------------------------------------------- |
| 201  | Created - Consumer credit record successfully created                |
| 409  | Conflict - A consumer credit record with id of `{id}` already exists |

### Request Body

#### consumer_facts

| Field             | Type              | Description                                                             |
| ----------------- | ----------------- | ----------------------------------------------------------------------- |
| first_name        | string            | First name of the consumer                                              |
| last_name         | string            | Last name of the consumer                                               |
| email             | string            | RFC 5322 and RFC 822 format email address of the consumer               |
| date_of_birth     | string            | ISO 8601 date format of the consumer's date of birth                    |
| address           | string            | CAN/CSA-Z109.1-01 or USPS Publication 28 address format of the consumer |
| phone_number      | string            | E.164 international format phone number of the consumer                 |
| SIN/SSN           | string (optional) | SIN(`NNN-NNN-NNN`) or SSN(`NNN-NN-NNNN`) of the consumer                |
| institution_names | array             | List of associated institutions. Each name must be between 2 and 50 characters. |

#### credit_facts

| Field                | Type   | Description                              |
| -------------------- | ------ | ---------------------------------------- |
| amount               | number | Amount requested by borrower, in dollars |
| credit_type          | string | Type of credit (`"PDL"` or `"BNPL"`)     |
| application_datetime | string | ISO 8601 datetime of application         |
| credit_state         | string | State of credit (see values below)       |

**Credit States**:

- `"application"`
- `"originated"`
- `"declined"`
- `"non-compliant"`
- `"compliant"`
- `"bankrupt/insolvent"`

**Example**:

```json
{
  "consumer_facts": {
    "first_name": "John",
    "last_name": "Doe",
    "email": "john.doe@example.com",
    "date_of_birth": "2010-10-10",
    "address": "101 1ST. S.W. Calgary AB T2P 2V6",
    "phone_number": "+11234567890",
    "institution_names": ["TD", "RBC"]
  },
  "credit_facts": {
    "amount": 1000,
    "credit_type": "PDL",
    "application_datetime": "2024-09-23 21:47:12.023476",
    "credit_state": "applied"
  }
}
```

:::info
The KlearSync Data ETL interface automatically populates new consumer credit records from your system into KlearLink, effectively eliminating the need to submit your records via API.

:::

---

## 2. Update a consumer credit record

**Endpoint**: `/consumer-credit/{id}`

**Method**: `POST`

**Description**: Update the consumer_facts and/or credit_facts of a previously submitted consumer credit record

**Path Parameters**:

| Parameter | Type   | Description                               |
| --------- | ------ | ----------------------------------------- |
| id        | string | ID of the existing consumer credit record |

**Response Codes**:

| Code | Description                                          |
| ---- | ---------------------------------------------------- |
| 200  | OK - Consumer credit record successfully updated     |
| 404  | Not Found - No consumer match found for id of `{id}` |

**Request Body**:

Same schema as Submit endpoint, with additional optional fields in credit_facts:

| Field               | Type   | Description                           |
| ------------------- | ------ | ------------------------------------- |
| originated_datetime | string | ISO 8601 datetime of origination      |
| payment_due_date    | string | ISO 8601 datetime of payment due date |
| payment_amount_due  | number | Amount due for payment, in dollars    |

**Example**:

```json
{
  "consumer_facts": {
    "first_name": "John",
    "last_name": "Doe",
    "email": "john.doe@example.com",
    "date_of_birth": "2010-10-10",
    "address": "101 1ST. S.W. Calgary AB T2P 2V6",
    "phone_number": "+11234567890",
    "institution_names": ["TD", "RBC"]
  },
  "credit_facts": {
    "amount": 1000,
    "credit_type": "PDL",
    "application_datetime": "2024-09-23 21:47:12.023476",
    "originated_datetime": "2024-09-24 15:43:12.023476",
    "payment_due_date": "2024-09-30 15:43:12.023476",
    "payment_amount_due": 1000,
    "credit_state": "originated"
  }
}
```

:::info
The KlearSync Data ETL interface will automatically update your existing consumer credit records in KlearLink, effectively eliminating the need to update them via API.

:::

---

## 3. View a submitted consumer credit record

**Endpoint**: `/consumer-credit/{id}`

**Method**: `GET`

**Description**: View the data associated with a submitted consumer credit record

**Path Parameters**:

| Parameter | Type   | Description                      |
| --------- | ------ | -------------------------------- |
| id        | string | ID of the consumer credit record |

**Response Codes**:

| Code | Description                                          |
| ---- | ---------------------------------------------------- |
| 200  | OK - Request successful                              |
| 404  | Not Found - No consumer match found for id of `{id}` |

**Response Body**:

Includes all fields from consumer_facts and credit_facts, plus:

| Field      | Type    | Description                          |
| ---------- | ------- | ------------------------------------ |
| created_at | string  | ISO 8601 datetime of record creation |
| updated_at | string  | ISO 8601 datetime of last update     |
| processed  | boolean | Whether record has been processed    |

**Example**:

```json
{
  "consumer_facts": {
    "first_name": "John",
    "last_name": "Doe",
    "email": "john.doe@example.com",
    "date_of_birth": "2010-10-10",
    "address": "101 1ST. S.W. Calgary AB T2P 2V6",
    "phone_number": "+11234567890",
    "institution_names": ["TD", "RBC"]
  },
  "credit_facts": {
    "amount": 1000,
    "credit_type": "PDL",
    "application_datetime": "2024-09-23 21:47:12.023476",
    "originated_datetime": "2024-09-24 15:43:12.023476",
    "payment_due_date": "2024-09-30 15:43:12.023476",
    "payment_amount_due": 1000,
    "credit_state": "originated"
  },
  "created_at": "datetime",
  "updated_at": "datetime",
  "processed": true
}
```

---

## 4. View Consumer Match

**Endpoint**: `/consumer-credit/{id}/consumer-match`

**Method**: `GET`

**Description**: View the results of inter-organizational consumer match results on your previously submitted consumer-credit

**Path Parameters**:

| Parameter | Type   | Description                      |
| --------- | ------ | -------------------------------- |
| id        | string | ID of the consumer credit record |

**Response Codes**:

| Code | Description                                          |
| ---- | ---------------------------------------------------- |
| 200  | OK - Consumer match identified                       |
| 404  | Not Found - No consumer match found for id of `{id}` |

**Response Body**:

Includes consumer_facts and credit_facts from original record, plus:

#### matched_on

| Field             | Type    | Description                              |
| ----------------- | ------- | ---------------------------------------- |
| first_name        | boolean | Whether first name matched               |
| last_name         | boolean | Whether last name matched                |
| email             | boolean | Whether email matched                    |
| date_of_birth     | boolean | Whether date of birth matched            |
| address           | boolean | Whether address matched                  |
| phone_number      | boolean | Whether phone number matched             |
| institution_names | array   | List of institutions from matched record |

**Example**:

```json
{
  "consumer_facts": {
    "first_name": "John",
    "last_name": "Doe",
    "email": "john.doe@example.com",
    "date_of_birth": "2010-10-10",
    "address": "101 1ST. S.W. Calgary AB T2P 2V6",
    "phone_number": "+11234567890",
    "institution_names": ["TD", "RBC"]
  },
  "credit_facts": {
    "amount": 1000,
    "credit_type": "PDL",
    "application_datetime": "2024-09-23 21:47:12.023476",
    "originated_datetime": "2024-09-24 15:43:12.023476",
    "payment_due_date": "2024-09-30 15:43:12.023476",
    "payment_amount_due": 1000,
    "credit_state": "originated"
  },
  "created_at": "datetime",
  "updated_at": "datetime",
  "processed": true,
  "consumer_match": [
    {
      "matched_on": {
        "first_name": true,
        "last_name": true,
        "email": true,
        "date_of_birth": true,
        "address": true,
        "phone_number": false,
        "institution_names": ["CIBC"]
      },
      "credit_facts": {
        "amount": 1200,
        "credit_type": "PDL",
        "application_datetime": "2024-09-23 11:47:12.023476",
        "originated_datetime": "2024-09-24 12:43:12.023476",
        "payment_due_date": "2024-09-30 07:43:12.023476",
        "payment_amount_due": 1200,
        "credit_state": "non-compliant"
      }
    }
  ]
}
```

:::info
Here, we see an inter-organizational match indicating that your applicant is non-compliant on a loan originated by another organization.

You do not see what organization the non-compliant loan originated from, nor do you obtain any additional information on the organization, nor do you see any consumer_facts or credit_facts that you do not already have.

:::

:::info
For real-time updates on consumer matches, use the KlearWatch interface.

:::

---

## Appendix

### A. Definitions

| Term              | Definition                                                                                                                           |
| ----------------- | ------------------------------------------------------------------------------------------------------------------------------------ |
| Consumer Credit   | A financial product extended to consumers for personal use, including both Pay Day Loans (PDL) and Buy Now Pay Later (BNPL) products |
| Consumer Facts    | Identifying information about a consumer that can be used for matching and verification purposes                                     |
| Credit Facts      | Details about a specific credit product, including amount, type, and current state                                                   |
| Institution Names | Financial institutions or lenders that have a relationship with the consumer                                                         |
| Consumer State    | The current status of a consumer in relation to their credit products                                                                |
| Credit State      | The current status of a specific credit product                                                                                      |

### B. Data Standards

| Data Type        | Standard            | Format Example                        | Description                                              |
| ---------------- | ------------------- | ------------------------------------- | -------------------------------------------------------- |
| First Name       | string              | `John`                                | Must be at least 2 characters long.                         |
| Last Name        | string              | `Doe`                                 | Must be at least 2 characters long.                         |
| Date             | ISO 8601            | `YYYY-MM-DD`                          | International date format. Dates must not be in the future. |
| Email            | RFC 5322/822        | `john.doe@example.com`                | RFC 5322 and RFC 822 format email address. Must be between 5 and 254 characters. |
| DateTime         | ISO 8601            | `YYYY-MM-DD HH:mm:ss.SSSSSS`          | International datetime format with microsecond precision. Must not be in the future. |
| Phone Number     | E.164               | `+1XXXXXXXXXX`                        | International phone number format. Must be between 10 and 15 digits. |
| Address (Canada) | CAN/CSA-Z109.1-01   | `101 1ST. S.W. Calgary AB T2P 2V6`    | Canadian postal address format. Must be between 5 and 100 characters. |
| Address (USA)    | USPS Publication 28 | `1234 MAIN ST NW WASHINGTON DC 20500` | US postal address format. Must be between 5 and 100 characters. |
| SIN              | CRA Standard        | `NNN-NNN-NNN`                         | Canadian Social Insurance Number format. Must be exactly 9 digits. Optional. |
| SSN              | SSA Standard        | `NNN-NN-NNNN`                         | US Social Security Number format. Must be exactly 9 digits. Optional. |

:::warn
The klearlink API has very strict data validation! All data sent to klearlink must be valid json and the aforementioned key fields MUST adhere to the specified format.
:::

:::info
All monetary values in this API are expressed in the local currency (CAD for Canadian transactions, USD for US transactions) and should be provided as decimal numbers with up to 2 decimal places.
:::

#### 1. E.164 Phone Number Validation

##### **Regex Pattern:**

```regex
^\+?[1-9]\d{1,14}$
```

##### **Description:**

This regex validates phone numbers following the **E.164 international standard**, ensuring they are globally unique and properly formatted.

##### **Rules:**

- The number may start with an optional `+`.
- The country code must be between 1 and 3 digits and cannot start with 0.
- The total length (including country code) must be between 2 and 15 digits.
- Only numeric digits are allowed (no spaces, dashes, or special characters apart from `+`).

##### **Examples:**

✅ Valid:

- `+12025550123`
- `+442071838750`
- `+919876543210`
- `+8613800138000`

❌ Invalid:

- `12025550123` (missing `+` but may be valid in certain systems)
- `+0123456789` (country code cannot start with 0)
- `+9999999999999999` (exceeds 15 digits)
- `+44 207 183 8750` (contains spaces)
- `+1-202-555-0123` (contains dashes)

---

#### 2. CAN/CSA-Z109.1-01 Canadian Address Validation

##### **Regex Pattern:**

```regex
^\d+\s[A-Za-z0-9\s.,'-]+,\s[A-Za-z\s-]+,\s(?:AB|BC|MB|NB|NL|NS|NT|NU|ON|PE|QC|SK|YT),\s[A-Za-z]\d[A-Za-z]\s?\d[A-Za-z]\d(?:,\sCanada)?$
```

##### **Description:**

This regex validates addresses formatted according to the **CAN/CSA-Z109.1-01** standard, which is commonly used in Canada.

##### **Rules:**

- The address must start with a **street number**.
- The **street name** must contain letters, numbers, spaces, and optional characters (`.,'-`).
- The **city name** must contain only letters and spaces.
- The **province/territory code** must be one of the valid two-letter abbreviations: `AB, BC, MB, NB, NL, NS, NT, NU, ON, PE, QC, SK, YT`.
- The **postal code** must follow the format `A1A 1A1` (where `A` is a letter and `1` is a digit) with an optional space.
- The **country name "Canada"** is optional.

##### **Examples:**

✅ Valid:

- `123 Main St, Toronto, ON, M5V 3L9`
- `4567 Elm Ave, Vancouver, BC, V6B 1H2, Canada`
- `77-101 King St W, Hamilton, ON, L8P 1A1`

❌ Invalid:

- `123 Main St Toronto ON M5V 3L9` (missing commas)
- `4567 Elm Ave, Vancouver, BC, 12345` (invalid postal code format)
- `Main St, Toronto, ON, M5V 3L9` (missing street number)
- `123 Fake St, Springfield, XX, M1M 1M1` (invalid province code)

---

#### 3. RFC 5322/822 Email Address Validation

##### **Regex Pattern:**

```regex
^[^\s@]+@[^\s@]+\.[^\s@]+$
```

### **Description:**

This regex validates email addresses according to the **RFC 5322/822** standard, ensuring that they conform to typical email formatting rules.

### **Rules:**

- The local part may contain alphanumeric characters, dots, and special characters (`!#$%&'*+/=?^_`{|}~-`).
- The local part may be enclosed in quotes (`"..."`) if special characters are used.
- The domain must contain alphanumeric characters and hyphens, but not start or end with a hyphen.
- The domain must end with a valid top-level domain (2-63 characters in length).

### **Examples:**

✅ Valid:

- `example@email.com`
- `user.name+tag@example.co.uk`
- `"quoted@text"@example.com`

❌ Invalid:

- `plainaddress` (missing `@` and domain)
- `@missinglocal.com` (missing local part)
- `user@.com` (invalid domain format)
- `user@com` (top-level domain too short)
- `user@-example.com` (hyphen at the start of domain)

---

#### 4. SIN Validation

### **Description:**

This validator ensures compliance with CRA standards for SIN numbers

### **Rules:**

- If None → Accept it (SIN is optional).
- If provided:
  - Must be 9 digits long.
  - Must start with 1-9 (no leading zero).
  - Must pass Luhn checksum validation.

### Error Messages

- "Invalid SIN: XXX. Must be exactly 9 digits."
- "Invalid SIN: XXX. Cannot start with 0."
- "Invalid SIN: XXX. Failed Luhn checksum validation."

---

:::info
Data validation errors are expressed in the following [format](https://docs.rs/serde_valid/latest/serde_valid/#validation-errors-format)
:::
