<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Flexpay Sms for completed Purchases with invalid phone numbers</name>
   <tag></tag>
   <elementGuidId>2974805c-3847-4f90-b8c0-a5b31d20db33</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;contentType&quot;: &quot;application/x-www-form-urlencoded&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;,
  &quot;parameters&quot;: [
    {
      &quot;name&quot;: &quot;recipients&quot;,
      &quot;value&quot;: &quot;wertyu&quot;
    },
    {
      &quot;name&quot;: &quot;template_id&quot;,
      &quot;value&quot;: &quot;2&quot;
    },
    {
      &quot;name&quot;: &quot;first_name&quot;,
      &quot;value&quot;: &quot;george&quot;
    },
    {
      &quot;name&quot;: &quot;last_name&quot;,
      &quot;value&quot;: &quot;njau&quot;
    },
    {
      &quot;name&quot;: &quot;product_name&quot;,
      &quot;value&quot;: &quot;smart watch&quot;
    },
    {
      &quot;name&quot;: &quot;product_price&quot;,
      &quot;value&quot;: &quot;2999&quot;
    },
    {
      &quot;name&quot;: &quot;outlet&quot;,
      &quot;value&quot;: &quot;imara&quot;
    },
    {
      &quot;name&quot;: &quot;amount_paid&quot;,
      &quot;value&quot;: &quot;1200&quot;
    },
    {
      &quot;name&quot;: &quot;phone_no&quot;,
      &quot;value&quot;: &quot;ertyuio&quot;
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
