<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>payment received sms template</description>
   <name>sms Template for Payment Received with invalid date format</name>
   <tag></tag>
   <elementGuidId>a0c4aade-8d9d-44f9-8a19-0c5f2380e520</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;contentType&quot;: &quot;application/x-www-form-urlencoded&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;,
  &quot;parameters&quot;: [
    {
      &quot;name&quot;: &quot;recipients&quot;,
      &quot;value&quot;: &quot;254722310817&quot;
    },
    {
      &quot;name&quot;: &quot;template_id&quot;,
      &quot;value&quot;: &quot;3&quot;
    },
    {
      &quot;name&quot;: &quot;first_name&quot;,
      &quot;value&quot;: &quot;kaizen&quot;
    },
    {
      &quot;name&quot;: &quot;last_name&quot;,
      &quot;value&quot;: &quot;njau&quot;
    },
    {
      &quot;name&quot;: &quot;product_name&quot;,
      &quot;value&quot;: &quot;sony tv&quot;
    },
    {
      &quot;name&quot;: &quot;outlet&quot;,
      &quot;value&quot;: &quot;Friesen Group&quot;
    },
    {
      &quot;name&quot;: &quot;amount_paid&quot;,
      &quot;value&quot;: &quot;1300&quot;
    },
    {
      &quot;name&quot;: &quot;phone_no&quot;,
      &quot;value&quot;: &quot;0719725060&quot;
    },
    {
      &quot;name&quot;: &quot;date&quot;,
      &quot;value&quot;: &quot;45-12-2018&quot;
    },
    {
      &quot;name&quot;: &quot;flex_amount&quot;,
      &quot;value&quot;: &quot;1300&quot;
    },
    {
      &quot;name&quot;: &quot;balance&quot;,
      &quot;value&quot;: &quot;5200&quot;
    }
  ]
}</httpBodyContent>
   <httpBodyType>x-www-form-urlencoded</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/x-www-form-urlencoded</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>http://176.58.103.227:8086/api/send_sms</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
