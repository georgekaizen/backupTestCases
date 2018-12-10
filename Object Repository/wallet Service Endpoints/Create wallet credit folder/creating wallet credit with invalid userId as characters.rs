<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>wallet credit amount</description>
   <name>creating wallet credit with invalid userId as characters</name>
   <tag></tag>
   <elementGuidId>01d1c501-21c6-4b52-94e5-21351bfa1bb4</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;contentType&quot;: &quot;application/x-www-form-urlencoded&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;,
  &quot;parameters&quot;: [
    {
      &quot;name&quot;: &quot;userId&quot;,
      &quot;value&quot;: &quot;vbnhghjk&quot;
    },
    {
      &quot;name&quot;: &quot;moneyInId&quot;,
      &quot;value&quot;: &quot;1&quot;
    },
    {
      &quot;name&quot;: &quot;amountCredited&quot;,
      &quot;value&quot;: &quot;1200&quot;
    },
    {
      &quot;name&quot;: &quot;amountSource&quot;,
      &quot;value&quot;: &quot;mpesa&quot;
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
   <restUrl>http://176.58.103.227:8084/api/flexpay/wallet/credit</restUrl>
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
